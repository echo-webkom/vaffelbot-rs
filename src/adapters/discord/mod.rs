pub mod commands;

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use poise::FrameworkOptions;
use serenity::Error as SerenityError;
use serenity::all::{GatewayIntents, GuildId, RoleId};

use crate::domain::{OrderRepository, QueueRepository};

const PREFIX: &str = "!";

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

pub struct Data {
    pub queue: Arc<dyn QueueRepository>,
    pub orders: Arc<dyn OrderRepository>,
    pub oracle_roles: RwLock<HashMap<GuildId, RoleId>>,
}

pub struct DiscordAdapter {
    token: String,
    queue: Arc<dyn QueueRepository>,
    orders: Arc<dyn OrderRepository>,
}

impl DiscordAdapter {
    pub fn new(
        token: String,
        queue: Arc<dyn QueueRepository>,
        orders: Arc<dyn OrderRepository>,
    ) -> Self {
        Self {
            token,
            queue,
            orders,
        }
    }

    pub async fn start(self) -> Result<(), SerenityError> {
        let options: FrameworkOptions<Data, Error> = poise::FrameworkOptions {
            commands: vec![
                commands::bake::bake(),
                commands::close::close(),
                commands::open::open(),
                commands::ping::ping(),
                commands::queue_size::queue(),
                commands::waffle::waffle(),
            ],
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some(PREFIX.into()),
                ..Default::default()
            },
            ..Default::default()
        };

        let framework = poise::Framework::builder()
            .setup(move |ctx, _ready, framework| {
                Box::pin(async move {
                    poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                    Ok(Data {
                        queue: self.queue.clone(),
                        orders: self.orders.clone(),
                        oracle_roles: RwLock::new(HashMap::new()),
                    })
                })
            })
            .options(options)
            .build();

        let mut client = serenity::Client::builder(
            self.token.clone(),
            GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT,
        )
        .framework(framework)
        .await?;

        client.start().await
    }
}

pub async fn check_is_oracle(ctx: Context<'_>) -> Result<bool, Error> {
    let guild_id = match ctx.guild_id() {
        Some(id) => id,
        None => {
            deny(ctx).await?;
            return Ok(false);
        }
    };

    // For slash commands, the member is already in the interaction data (no API call).
    let member = match ctx.author_member().await {
        Some(member) => member,
        None => {
            deny(ctx).await?;
            return Ok(false);
        }
    };

    // Check cache for the orakel role ID
    let cached = ctx
        .data()
        .oracle_roles
        .read()
        .unwrap()
        .get(&guild_id)
        .copied();

    let orakel_role_id = match cached {
        Some(id) => id,
        None => {
            let roles = match guild_id.roles(ctx).await {
                Ok(roles) => roles,
                Err(_) => {
                    deny(ctx).await?;
                    return Ok(false);
                }
            };

            match roles.values().find(|r| r.name.to_lowercase() == "orakel") {
                Some(role) => {
                    let id = role.id;
                    ctx.data()
                        .oracle_roles
                        .write()
                        .unwrap()
                        .insert(guild_id, id);
                    id
                }
                None => {
                    deny(ctx).await?;
                    return Ok(false);
                }
            }
        }
    };

    if member.roles.contains(&orakel_role_id) {
        return Ok(true);
    }

    deny(ctx).await?;
    Ok(false)
}

async fn deny(ctx: Context<'_>) -> Result<(), Error> {
    // Send message to discord to prevent timeout.
    // Discord expects a response within 3 seconds. Just
    // returning false does not respond to the interaction.
    ctx.send(
        poise::CreateReply::default()
            .content("❌ Du har ikke tilgang til denne kommandoen.")
            .ephemeral(true),
    )
    .await?;
    Ok(())
}
