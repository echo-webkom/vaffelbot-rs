use serenity::Error as SerenityError;
use std::sync::Arc;

use poise::FrameworkOptions;
use serenity::all::GatewayIntents;

use crate::{commands, queue::WaffleQueue};

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

pub struct Data {
    pub queue: Arc<WaffleQueue>,
}

pub struct WaffleBot {
    token: String,
    queue: Arc<WaffleQueue>,
}

impl WaffleBot {
    pub fn new(token: String, queue: Arc<WaffleQueue>) -> Self {
        Self { token, queue }
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
                prefix: Some("!".into()),
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
    if let Ok(member) = ctx.guild_id().unwrap().member(ctx, ctx.author().id).await {
        if let Some(guild_id) = ctx.guild_id() {
            if let Ok(roles) = guild_id.roles(ctx).await {
                if let Some(orakel_role_id) = roles
                    .values()
                    .find(|r| r.name.to_lowercase() == "orakel")
                    .map(|r| r.id)
                {
                    return Ok(member.roles.contains(&orakel_role_id));
                }
            }
        }
    }

    Ok(false)
}
