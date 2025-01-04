use std::{borrow::BorrowMut as _, future::Future, pin::Pin, sync::Arc};

use tokio::{
    sync::mpsc::{self, UnboundedReceiver, UnboundedSender},
    task::JoinHandle,
};

use tokio_util::{sync::CancellationToken, task::TaskTracker};
use tulpje_shared::DiscordEventMeta;
use twilight_gateway::Event;
use twilight_http::Client;
use twilight_model::id::{marker::ApplicationMarker, Id};

use crate::{Context, Error, Registry, Scheduler};

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

    pub fn build(&self) -> (Framework<T>, UnboundedSender<(DiscordEventMeta, Event)>) {
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
    registry: Arc<Registry<T>>,
    client: Arc<Client>,
    scheduler: Scheduler,

    app_id: Id<ApplicationMarker>,
    user_data: Arc<T>,

    setup_fn: Option<SetupFunc<T>>,
    sched_handle: Option<JoinHandle<()>>,

    shutdown_token: CancellationToken,
    dispatcher_handle: JoinHandle<()>,
}

impl<T: Clone + Send + Sync + 'static> Framework<T> {
    pub fn new(
        registry: Arc<Registry<T>>,
        client: Arc<Client>,
        app_id: Id<ApplicationMarker>,
        user_data: Arc<T>,
        setup_fn: Option<SetupFunc<T>>,
    ) -> (Self, UnboundedSender<(DiscordEventMeta, Event)>) {
        let (sender, receiver) = mpsc::unbounded_channel();
        let shutdown_token = CancellationToken::new();

        let mut dispatcher = Dispatcher::new(
            Arc::clone(&registry),
            Arc::clone(&client),
            app_id,
            Arc::clone(&user_data),
            receiver,
            shutdown_token.clone(),
        );
        let dispatcher_handle = tokio::spawn(async move {
            dispatcher.run().await;
        });

        (
            Self {
                registry,
                client,
                scheduler: Scheduler::new(),

                app_id,
                user_data,

                setup_fn,
                sched_handle: None,

                shutdown_token,
                dispatcher_handle,
            },
            sender,
        )
    }

    pub async fn start(&mut self) -> Result<(), Error> {
        let ctx = Context {
            application_id: self.app_id,
            services: Arc::clone(&self.user_data),
            client: Arc::clone(&self.client),
        };

        self.sched_handle = Some(
            self.scheduler
                .run(ctx.clone(), self.registry.tasks.values().collect())
                .await,
        );

        if let Some(setup_fn) = self.setup_fn.take() {
            (setup_fn)(ctx.clone())
                .await
                .map_err(|err| format!("error running setup function: {}", err))?;
        }

        Ok(())
    }

    pub async fn join(&mut self) -> Result<(), Error> {
        self.dispatcher_handle.borrow_mut().await?;
        if let Some(sched_handle) = self.sched_handle.take() {
            sched_handle.await?;
        }

        Ok(())
    }

    pub async fn shutdown(&mut self) {
        self.shutdown_token.cancel();
        self.scheduler.shutdown().await;
    }
}

struct Dispatcher<T: Clone + Send + Sync> {
    registry: Arc<Registry<T>>,
    client: Arc<Client>,
    app_id: Id<ApplicationMarker>,
    user_data: Arc<T>,

    receiver: UnboundedReceiver<(DiscordEventMeta, Event)>,
    shutdown_token: CancellationToken,
}

impl<T: Clone + Send + Sync + 'static> Dispatcher<T> {
    pub fn new(
        registry: Arc<Registry<T>>,
        client: Arc<Client>,
        app_id: Id<ApplicationMarker>,
        user_data: Arc<T>,

        receiver: UnboundedReceiver<(DiscordEventMeta, Event)>,
        shutdown_token: CancellationToken,
    ) -> Self {
        Self {
            registry,
            client,
            app_id,
            user_data,

            receiver,
            shutdown_token,
        }
    }

    pub async fn run(&mut self) {
        let ctx = Context {
            application_id: self.app_id,
            services: Arc::clone(&self.user_data),
            client: Arc::clone(&self.client),
        };

        let tracker = TaskTracker::new();

        loop {
            tokio::select! {
                data = self.receiver.recv() => {
                    let Some((meta, event)) = data else {
                        tracing::info!("received empty event, exiting...");
                        break;
                    };

                    let registry = Arc::clone(&self.registry);
                    let ctx = ctx.clone();

                    tracker.spawn(async move {
                        crate::handle(meta, ctx, &registry, event).await;
                    });
                },
                () = self.shutdown_token.cancelled() => self.receiver.close(),
            };
        }

        tracker.close();
        tracker.wait().await;
    }
}
