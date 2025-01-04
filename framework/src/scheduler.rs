use std::{collections::HashMap, future::Future, pin::Pin};

use async_cron_scheduler::{Job, JobId, Scheduler as CronScheduler};
use chrono::Utc;

use crate::{
    context::{Context, TaskContext},
    handler::task_handler::TaskHandler,
};

struct Runner {
    scheduler: Option<CronScheduler<Utc>>,
    service: Option<Pin<Box<dyn Future<Output = ()> + Send + Sync>>>,
}

impl Runner {
    fn init(&mut self) {
        assert!(
            self.scheduler.is_none(),
            "don't call init twice unless we've stopped previously"
        );

        let (scheduler, service) = CronScheduler::<Utc>::launch(tokio::time::sleep);

        self.scheduler = Some(scheduler);
        self.service = Some(Box::pin(service));
    }
    // create the scheduler and service if it doesn't exist yet and then return it
    fn get_scheduler(&mut self) -> &mut CronScheduler<Utc> {
        if self.scheduler.is_none() {
            self.init();
        }

        self.scheduler.as_mut().expect("we just assigned it")
    }

    fn start(&mut self) -> tokio::task::JoinHandle<()> {
        if self.scheduler.is_none() {
            self.init();
        }

        tokio::spawn(self.service.take().unwrap())
    }

    fn started(&self) -> bool {
        self.scheduler.is_some() && self.service.is_none()
    }

    async fn stop(&mut self, jobs: Vec<JobId>) {
        self.service.take();

        let Some(mut scheduler) = self.scheduler.take() else {
            return;
        };

        for job in jobs {
            scheduler.remove(job).await;
        }
    }
}

pub struct Scheduler {
    job_map: HashMap<String, JobId>,
    runner: Runner,
}

impl Scheduler {
    #[expect(
        clippy::new_without_default,
        reason = "we might have constructor arguments in the future, having a Default implementation feels incorrect"
    )]
    pub fn new() -> Self {
        Self {
            job_map: HashMap::new(),
            runner: Runner {
                scheduler: None,
                service: None,
            },
        }
    }

    pub async fn enable_task<T: Clone + Send + Sync + 'static>(
        &mut self,
        ctx: Context<T>,
        task: TaskHandler<T>,
    ) {
        let job = Job::<Utc>::cron_schedule(task.cron.clone());
        let job_name = task.name.clone();
        let job_id = self
            .runner
            .get_scheduler()
            .insert(job, move |_id| {
                let job_ctx = ctx.clone();
                let job_handler = task.clone();

                tokio::spawn(async move {
                    if let Err(err) = job_handler.run(TaskContext::from_context(job_ctx)).await {
                        tracing::error!("error running task {}: {}", job_handler.name, err);
                    };
                });
            })
            .await;

        self.job_map.insert(job_name, job_id);
    }

    pub async fn disable_task(&mut self, name: &str) {
        let Some(job_id) = self.job_map.remove(name) else {
            return;
        };

        self.runner.get_scheduler().remove(job_id).await;
    }

    pub async fn run<T: Clone + Send + Sync + 'static>(
        &mut self,
        ctx: Context<T>,
        tasks: Vec<&TaskHandler<T>>,
    ) -> tokio::task::JoinHandle<()> {
        assert!(
            !self.runner.started(),
            "Scheduler::run called twice, scheduler is already running"
        );

        for task in tasks {
            self.enable_task(ctx.clone(), task.clone()).await;
        }

        self.runner.start()
    }

    // drain the jobs from the job map and also take the scheduler
    // removing the jobs, and taking the scheduler from the runner should
    // cause the scheduler to be dropped and thus stop
    pub async fn shutdown(&mut self) {
        let jobs: Vec<JobId> = self.job_map.drain().map(|(_k, v)| v).collect();
        self.runner.stop(jobs).await;
    }
}
