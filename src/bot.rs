use serenity::all::UserId;
use serenity::async_trait;
use serenity::model::application::Interaction;
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::prelude::*;

use crate::commands::{
    BakeCommand, CloseCommand, EmptyCommand, OpenCommand, PingCommand, QueueSizeCommand,
    WaffleCommand,
};
use crate::queue::WaffleQueue;

pub struct WaffleBot {
    token: String,
    guild_id: GuildId,
    queue: WaffleQueue,
}

impl WaffleBot {
    pub fn new(token: String, guild_id: u64, queue: WaffleQueue) -> Self {
        let guild_id = GuildId::new(guild_id);

        Self {
            token,
            guild_id,
            queue,
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
        let mut client = Client::builder(&self.token.clone(), GatewayIntents::empty())
            .event_handler(self)
            .await?;

        client.start().await
    }
}

#[async_trait]
impl EventHandler for WaffleBot {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            let user_id = command.user.id.to_string();
            let is_oracle = self.is_user_oracle(&ctx, command.user.id).await;

            let content = match command.data.name.as_str() {
                "ping" => Some(PingCommand::run()),
                "kø" => Some(QueueSizeCommand::run(&self.queue, user_id)),
                "vaffel" => Some(WaffleCommand::run(&self.queue, user_id)),
                "stekt" => Some(BakeCommand::run(
                    &self.queue,
                    command.data.options[0].value.as_i64(),
                    is_oracle,
                )),
                "start" => Some(OpenCommand::run(&self.queue, is_oracle)),
                "stopp" => Some(CloseCommand::run(&self.queue, is_oracle)),
                "tøm" => Some(EmptyCommand::run(&self.queue, is_oracle)),
                _ => None,
            };

            if let Some(content) = content {
                if let Err(why) = command.create_response(&ctx.http, content).await {
                    println!("Cannot respond to slash command: {why}");
                }
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let commands = self
            .guild_id
            .set_commands(
                &ctx.http,
                vec![
                    PingCommand::register(),
                    QueueSizeCommand::register(),
                    WaffleCommand::register(),
                    BakeCommand::register(),
                    OpenCommand::register(),
                    CloseCommand::register(),
                    EmptyCommand::register(),
                ],
            )
            .await;

        if let Err(why) = commands {
            println!("Error registering commands: {why}");
        } else {
            println!("Commands registered successfully");
        }
    }
}
