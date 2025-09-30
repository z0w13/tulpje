use std::{future::Future, pin::Pin, sync::Arc};

use tokio::{sync::mpsc, task::JoinHandle};

use crate::Metadata;
use tokio_util::{sync::CancellationToken, task::TaskTracker};
use twilight_gateway::Event;
use twilight_http::Client;
use twilight_model::id::{Id, marker::ApplicationMarker};

use crate::handler::task_handler::TaskHandler;
use crate::scheduler::{SchedulerHandle, SchedulerTaskMessage};
use crate::{Context, Error, Registry};

type SetupFunc<T> = fn(ctx: Context<T>) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>>;

#[derive(Clone)]
pub struct FrameworkBuilder<T: Clone + Send + Sync> {
    registry: Arc<Registry<T>>,
    client: Arc<Client>,
    app_id: Id<ApplicationMarker>,
    user_data: Arc<T>,

    setup_fn: Option<SetupFunc<T>>,
}

impl<T: Clone + Send + Sync + 'static> FrameworkBuilder<T> {
    pub fn new(
        registry: Arc<Registry<T>>,
        client: Client,
        app_id: Id<ApplicationMarker>,
        user_data: T,
    ) -> Self {
        Self {
            registry,
            client: Arc::new(client),
            app_id,
            user_data: Arc::new(user_data),
            setup_fn: None,
        }
    }

    pub fn setup(&mut self, func: SetupFunc<T>) -> &mut Self {
        self.setup_fn = Some(func);
        self
    }

    pub fn build(&self) -> Framework<T> {
        Framework::new(
            Arc::clone(&self.registry),
            Arc::clone(&self.client),
            self.app_id,
            Arc::clone(&self.user_data),
            self.setup_fn,
        )
    }
}

pub struct Framework<T: Clone + Send + Sync> {
    ctx: Context<T>,
    setup_fn: Option<SetupFunc<T>>,

    scheduler: SchedulerHandle<T>,
    dispatcher: DispatchHandle,
}

impl<T: Clone + Send + Sync + 'static> Framework<T> {
    pub fn new(
        registry: Arc<Registry<T>>,
        client: Arc<Client>,
        application_id: Id<ApplicationMarker>,
        services: Arc<T>,
        setup_fn: Option<SetupFunc<T>>,
    ) -> Self {
        let ctx = Context {
            application_id,
            services,
            client,
        };
        let scheduler =
            SchedulerHandle::new(registry.tasks.values().cloned().collect(), ctx.clone());
        let dispatcher = DispatchHandle::new(registry, ctx.clone());

        Self {
            ctx,
            setup_fn,

            scheduler,
            dispatcher,
        }
    }

    pub async fn start(&mut self) -> Result<(), Error> {
        if let Some(setup_fn) = self.setup_fn.take() {
            (setup_fn)(self.ctx.clone())
                .await
                .map_err(|err| format!("error running setup function: {}", err))?;
        }

        self.scheduler
            .start()
            .map_err(|err| format!("error starting scheduled tasks: {}", err))?;

        Ok(())
    }

    pub fn enable_task(
        &mut self,
        handler: TaskHandler<T>,
    ) -> Result<(), Box<mpsc::error::SendError<SchedulerTaskMessage<T>>>> {
        self.scheduler.enable_task(handler)
    }

    pub fn disable_task(
        &mut self,
        name: String,
    ) -> Result<(), Box<mpsc::error::SendError<SchedulerTaskMessage<T>>>> {
        self.scheduler.disable_task(name)
    }

    pub fn sender(&self) -> Sender {
        Sender {
            sender: self.dispatcher.sender.clone(),
        }
    }

    pub fn send(
        &mut self,
        meta: Metadata,
        event: Event,
    ) -> Result<(), Box<mpsc::error::SendError<(Metadata, Event)>>> {
        self.dispatcher.send(meta, event)
    }

    pub async fn shutdown(&mut self) {
        self.scheduler.shutdown();
        self.dispatcher.shutdown();
    }

    pub async fn join(&mut self) -> Result<(), Error> {
        self.scheduler.join().await?;
        self.dispatcher.join().await?;

        Ok(())
    }
}

struct DispatchHandle {
    sender: mpsc::UnboundedSender<(Metadata, Event)>,
    shutdown: CancellationToken,
    handle: Option<JoinHandle<()>>,
}
impl DispatchHandle {
    fn new<T: Clone + Send + Sync + 'static>(registry: Arc<Registry<T>>, ctx: Context<T>) -> Self {
        let (sender, receiver) = mpsc::unbounded_channel();
        let shutdown = CancellationToken::new();

        let mut dispatch = Dispatch::new(ctx, registry, receiver, shutdown.child_token());
        let handle = Some(tokio::spawn(async move { dispatch.run().await }));

        Self {
            sender,
            shutdown,
            handle,
        }
    }

    fn send(
        &mut self,
        meta: Metadata,
        event: Event,
    ) -> Result<(), Box<mpsc::error::SendError<(Metadata, Event)>>> {
        Ok(self.sender.send((meta, event))?)
    }

    fn shutdown(&mut self) {
        self.shutdown.cancel();
    }

    async fn join(&mut self) -> Result<(), Error> {
        Ok(self
            .handle
            .take()
            .ok_or("Dispatch already shutdown")?
            .await?)
    }
}

struct Dispatch<T: Clone + Send + Sync> {
    registry: Arc<Registry<T>>,
    ctx: Context<T>,

    receiver: mpsc::UnboundedReceiver<(Metadata, Event)>,
    shutdown: CancellationToken,

    tracker: TaskTracker,
}
impl<T: Clone + Send + Sync + 'static> Dispatch<T> {
    fn new(
        ctx: Context<T>,
        registry: Arc<Registry<T>>,

        receiver: mpsc::UnboundedReceiver<(Metadata, Event)>,
        shutdown: CancellationToken,
    ) -> Self {
        Self {
            registry,
            ctx,

            receiver,
            shutdown,

            tracker: TaskTracker::new(),
        }
    }

    async fn run(&mut self) {
        loop {
            tokio::select! {
                Some((meta, event)) = self.receiver.recv() => {
                    let registry = Arc::clone(&self.registry);
                    let ctx = self.ctx.clone();

                    self.tracker.spawn(async move {
                        crate::handle(meta, ctx, &registry, event).await;
                    });
                },
                () = self.shutdown.cancelled() => break,
            }
        }

        self.receiver.close();
        self.tracker.close();

        self.tracker.wait().await;
    }
}

pub struct Sender {
    sender: mpsc::UnboundedSender<(Metadata, Event)>,
}

impl Sender {
    pub fn send(
        &self,
        meta: Metadata,
        event: Event,
    ) -> Result<(), Box<mpsc::error::SendError<(Metadata, Event)>>> {
        Ok(self.sender.send((meta, event))?)
    }

    pub fn closed(&self) -> bool {
        self.sender.is_closed()
    }
}
