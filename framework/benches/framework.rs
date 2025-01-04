use std::{sync::Arc, time::Duration};

use criterion::{criterion_group, criterion_main, Criterion};
use tokio::runtime::Runtime;
use twilight_gateway::{Event, EventTypeFlags};
use twilight_http::client::ClientBuilder;
use twilight_model::{
    application::command::CommandType,
    id::{marker::ApplicationMarker, Id},
};
use twilight_util::builder::command::CommandBuilder;

use tulpje_framework::{
    context::CommandContext, framework::FrameworkBuilder, handler_func, Error, ModuleBuilder,
    Registry,
};
use tulpje_shared::DiscordEvent;

#[derive(Clone)]
struct UserData {}

const EVENT_JSON: &str = r###"
    {
      "t": "INTERACTION_CREATE",
      "s": 3,
      "op": 0,
      "d": {
        "version": 1,
        "type": 2,
        "token": "empty",
        "member": {
          "user": {
            "username": "username",
            "public_flags": 0,
            "primary_guild": null,
            "id": "1",
            "global_name": "Zowie UwU",
            "discriminator": "0",
            "clan": null,
            "avatar_decoration_data": null,
            "avatar": "00000000000000000000000000000000"
          },
          "unusual_dm_activity_until": null,
          "roles": [ "1" ],
          "premium_since": null,
          "permissions": "1",
          "pending": false,
          "nick": "Nickname",
          "mute": false,
          "joined_at": "2020-03-10T00:00:00.000000+00:00",
          "flags": 0,
          "deaf": false,
          "communication_disabled_until": null,
          "banner": null,
          "avatar": null
        },
        "locale": "en-US",
        "id": "1",
        "guild_locale": "en-US",
        "guild_id": "1",
        "guild": {
          "locale": "en-US",
          "id": "1",
          "features": [
            "ENABLED_MODERATION_EXPERIENCE_FOR_NON_COMMUNITY",
            "COMMUNITY",
            "NEWS"
          ]
        },
        "entitlements": [],
        "entitlement_sku_ids": [],
        "data": {
          "type": 1,
          "options": [ ],
          "name": "command-name",
          "id": "1"
        },
        "context": 0,
        "channel_id": "1",
        "channel": {
          "type": 0,
          "topic": null,
          "theme_color": null,
          "rate_limit_per_user": 0,
          "position": 2,
          "permissions": "1",
          "parent_id": "1",
          "nsfw": false,
          "name": "general",
          "last_message_id": "1",
          "id": "1",
          "icon_emoji": {
            "name": "👋",
            "id": null
          },
          "guild_id": "1",
          "flags": 0
        },
        "authorizing_integration_owners": {
          "0": "1"
        },
        "application_id": "1",
        "app_permissions": "1"
      }
    }
"###;

fn benchmark_framework<T: Clone + Send + Sync + 'static>(
    c: &mut Criterion,
    name: &str,
    builder: &FrameworkBuilder<T>,
) {
    c.bench_function(name, |b| {
        b.to_async(Runtime::new().expect("error creating runtime"))
            .iter(|| {
                let mut framework = builder.build();
                let event = DiscordEvent::new(1, EVENT_JSON.to_string());
                let discord_event: Event =
                    twilight_gateway::parse(event.payload, EventTypeFlags::all())
                        .expect("Couldn't parse payload")
                        .expect("Payload is None")
                        .into();

                let sender = framework.sender();
                async move {
                    framework.start().await.expect("error starting framework");

                    for _iter in 0..100 {
                        if let Err(err) = sender.send(event.meta.clone(), discord_event.clone()) {
                            panic!("error queueing (closed={}): {}", sender.closed(), err);
                        }
                    }

                    framework.shutdown().await;
                    framework.join().await.expect("error shutting down");
                }
            });
    });
}

fn large_dispatch(c: &mut Criterion) {
    let user_data = UserData {};
    let client = ClientBuilder::new().build();
    let mut registry = Registry::<UserData>::new();

    async fn command_func(_ctx: CommandContext<UserData>) -> Result<(), Error> {
        Ok(())
    }

    let mut builder = ModuleBuilder::<UserData>::new("bench");
    for num in 0..1_000 {
        builder = builder.command(
            CommandBuilder::new(
                format!("command-name-{}", num),
                "desc",
                CommandType::ChatInput,
            )
            .build(),
            handler_func!(command_func),
        );
    }
    registry.register(builder.build());

    let builder = FrameworkBuilder::new(
        Arc::new(registry),
        client,
        Id::<ApplicationMarker>::new(1),
        user_data,
    );
    benchmark_framework(c, "large_dispatch", &builder);
}

fn slow_commands(c: &mut Criterion) {
    let user_data = UserData {};
    let client = ClientBuilder::new().build();
    let mut registry = Registry::<UserData>::new();

    async fn command_func(_ctx: CommandContext<UserData>) -> Result<(), Error> {
        tokio::time::sleep(Duration::from_millis(10)).await;
        Ok(())
    }

    let builder = ModuleBuilder::<UserData>::new("bench").command(
        CommandBuilder::new("command-name", "desc", CommandType::ChatInput).build(),
        handler_func!(command_func),
    );
    registry.register(builder.build());

    let builder = FrameworkBuilder::new(
        Arc::new(registry),
        client,
        Id::<ApplicationMarker>::new(1),
        user_data,
    );
    benchmark_framework(c, "slow_commands", &builder);
}

criterion_group! {
    name = throughput;
    config = Criterion::default().sample_size(10);
    targets = large_dispatch, slow_commands
}
criterion_main!(throughput);
