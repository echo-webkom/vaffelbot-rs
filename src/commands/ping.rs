use crate::bot::{Context, Error};

/// Ping vaffelbot
#[poise::command(prefix_command, slash_command, rename = "ping")]
#[tracing::instrument(name = "ping", skip(ctx))]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Pong!").await?;
    Ok(())
}
