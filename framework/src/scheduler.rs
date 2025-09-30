use std::collections::HashMap;

use async_cron_scheduler::{Job, JobId, Scheduler as CronScheduler};
use chrono::Utc;
use tokio::{sync::mpsc, task::JoinHandle};
use tokio_util::sync::CancellationToken;

use crate::{
    Error,
    context::{Context, TaskContext},
    handler::task_handler::TaskHandler,
};

pub enum SchedulerTaskMessage<T: Clone + Send + Sync> {
    Start(Vec<TaskHandler<T>>),
    Enable(TaskHandler<T>),
    Disable(String),
}

pub struct SchedulerHandle<T: Clone + Send + Sync> {
    tasks: Vec<TaskHandler<T>>,
    sender: mpsc::UnboundedSender<SchedulerTaskMessage<T>>,
    shutdown: CancellationToken,
    handle: Option<JoinHandle<()>>,
}
impl<T: Clone + Send + Sync + 'static> SchedulerHandle<T> {
    pub(crate) fn new(tasks: Vec<TaskHandler<T>>, ctx: Context<T>) -> Self {
        let (sender, receiver) = mpsc::unbounded_channel();
        let shutdown = CancellationToken::new();

        let mut scheduler = Scheduler::new(ctx, receiver, shutdown.clone());
        let handle = Some(tokio::spawn(async move { scheduler.run().await }));

        Self {
            tasks,
            sender,
            shutdown,
            handle,
        }
    }

    pub(crate) fn shutdown(&mut self) {
        self.shutdown.cancel();
    }

    pub(crate) fn start(
        &mut self,
    ) -> Result<(), Box<mpsc::error::SendError<SchedulerTaskMessage<T>>>> {
        Ok(self
            .sender
            .send(SchedulerTaskMessage::Start(self.tasks.clone()))?)
    }

    pub fn enable_task(
        &mut self,
        handler: TaskHandler<T>,
    ) -> Result<(), Box<mpsc::error::SendError<SchedulerTaskMessage<T>>>> {
        Ok(self.sender.send(SchedulerTaskMessage::Enable(handler))?)
    }

    pub fn disable_task(
        &mut self,
        name: String,
    ) -> Result<(), Box<mpsc::error::SendError<SchedulerTaskMessage<T>>>> {
        Ok(self.sender.send(SchedulerTaskMessage::Disable(name))?)
    }

    pub(crate) async fn join(&mut self) -> Result<(), Error> {
        Ok(self
            .handle
            .take()
            .ok_or("Scheduler already shutdown")?
            .await?)
    }
}

struct Scheduler<T: Clone + Send + Sync> {
    job_map: HashMap<String, JobId>,
    scheduler: Option<CronScheduler<Utc>>,
    handle: Option<JoinHandle<()>>,

    ctx: Context<T>,
    receiver: mpsc::UnboundedReceiver<SchedulerTaskMessage<T>>,
    shutdown: CancellationToken,
}

impl<T: Clone + Send + Sync + 'static> Scheduler<T> {
    fn new(
        ctx: Context<T>,
        receiver: mpsc::UnboundedReceiver<SchedulerTaskMessage<T>>,
        shutdown: CancellationToken,
    ) -> Self {
        let (scheduler, service) = CronScheduler::<Utc>::launch(tokio::time::sleep);

        Self {
            ctx,
            receiver,
            shutdown,

            job_map: HashMap::new(),
            scheduler: Some(scheduler),
            handle: Some(tokio::spawn(service)),
        }
    }

    pub async fn enable_task(&mut self, handler: TaskHandler<T>) {
        let local_ctx = self.ctx.clone();

        let job = Job::<Utc>::cron_schedule(handler.cron.clone());
        let job_name = handler.name.clone();
        let job_id = self
            .scheduler
            .as_mut()
            .unwrap()
            .insert(job, move |_id| {
                let job_ctx = local_ctx.clone();
                let job_handler = handler.clone();

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

        self.scheduler.as_mut().unwrap().remove(job_id).await;
    }

    async fn run(&mut self) {
        loop {
            tokio::select! {
                Some(msg) = self.receiver.recv() => {
                    match msg {
                        SchedulerTaskMessage::Start(tasks) => {
                            for task in tasks {
                                self.enable_task(task.clone()).await;
                            }
                        },
                        SchedulerTaskMessage::Enable(task) => self.enable_task(task).await,
                        SchedulerTaskMessage::Disable(name) => self.disable_task(&name).await,
                    }
                },
                () = self.shutdown.cancelled() => break,
            }
        }

        // drain the jobs from the job map and also take the scheduler
        // removing the jobs, and taking the scheduler from the runner should
        // cause the scheduler to be dropped and thus stop
        //
        // NOTE: Separate scope so we drop correctly after removing jobs
        {
            let Some(mut scheduler) = self.scheduler.take() else {
                tracing::warn!("Scheduler already removed");
                return;
            };

            for (_, job) in self.job_map.drain() {
                scheduler.remove(job).await;
            }
        }

        let Some(handle) = self.handle.take() else {
            tracing::warn!("CronScheduler already shutdown");
            return;
        };

        if let Err(err) = handle.await {
            tracing::warn!("Error joining CronScheduler: {}", err);
        }
    }
}
