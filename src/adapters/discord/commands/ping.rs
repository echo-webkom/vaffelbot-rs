use crate::adapters::discord::{Context, Error};

/// Ping vaffelbot
#[tracing::instrument(name = "ping", skip(ctx))]
#[poise::command(prefix_command, slash_command, rename = "ping")]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("🏓 Pong!").await?;
    Ok(())
}
