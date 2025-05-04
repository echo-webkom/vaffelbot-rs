use std::sync::Arc;

use serenity::all::UserId;
use serenity::async_trait;
use serenity::model::application::Interaction;
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::prelude::*;
use tracing::{debug, error, info};

use crate::commands::bake::BakeCommand;
use crate::commands::close::CloseCommand;
use crate::commands::empty::EmptyCommand;
use crate::commands::open::OpenCommand;
use crate::commands::ping::PingCommand;
use crate::commands::queue_size::QueueSizeCommand;
use crate::commands::waffle::WaffleCommand;
use crate::commands::CommandHandler;
use crate::queue::WaffleQueue;

pub struct WaffleContext<'a> {
    pub is_oracle: bool,
    pub queue: Arc<WaffleQueue>,
    #[allow(dead_code)]
    pub context: &'a Context,
}

pub struct WaffleBot {
    token: String,
    guild_id: GuildId,
    queue: Arc<WaffleQueue>,
    commands: Vec<Box<(dyn CommandHandler + 'static)>>,
}

impl WaffleBot {
    pub fn new(token: String, guild_id: u64, queue: Arc<WaffleQueue>) -> Self {
        let guild_id = GuildId::new(guild_id);

        let commands: Vec<Box<dyn CommandHandler>> = vec![
            Box::new(PingCommand::new()),
            Box::new(OpenCommand::new()),
            Box::new(QueueSizeCommand::new()),
            Box::new(WaffleCommand::new()),
            Box::new(BakeCommand::new()),
            Box::new(CloseCommand::new()),
            Box::new(EmptyCommand::new()),
        ];

        Self {
            token,
            guild_id,
            queue,
            commands,
        }
    }

    pub async fn is_user_oracle(&self, ctx: &Context, user_id: UserId) -> bool {
        if let Ok(member) = self.guild_id.member(ctx, user_id).await {
            if let Ok(roles) = self.guild_id.roles(ctx).await {
                if let Some(orakel_role_id) = roles
                    .values()
                    .find(|r| r.name.to_lowercase() == "orakel")
                    .map(|r| r.id)
                {
                    return member.roles.contains(&orakel_role_id);
                }
            }
        }

        false
    }

    pub async fn start(self) -> Result<(), serenity::Error> {
        let mut client = Client::builder(self.token.clone(), GatewayIntents::empty())
            .event_handler(self)
            .await?;

        client.start().await
    }
}

#[async_trait]
impl EventHandler for WaffleBot {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            let cmd = self
                .commands
                .iter()
                .find(|cmd| cmd.name() == command.data.name);

            if let Some(cmd) = cmd {
                let is_oracle = self.is_user_oracle(&ctx, command.user.id).await;
                let c = WaffleContext {
                    is_oracle,
                    queue: self.queue.clone(),
                    context: &ctx,
                };

                let content = cmd.execute(&c, &command);

                if let Err(why) = command.create_response(&ctx.http, content).await {
                    debug!("Cannot respond to slash command: {why}");
                }
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);

        let commands = self
            .guild_id
            .set_commands(
                &ctx.http,
                self.commands.iter().map(|cmd| cmd.register()).collect(),
            )
            .await;

        if let Err(why) = commands {
            error!("Error registering commands: {why}");
        } else {
            info!("Commands registered successfully");
        }
    }
}
