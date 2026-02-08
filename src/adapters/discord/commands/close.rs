use serenity::all::OnlineStatus;

use crate::adapters::discord::{check_is_oracle, Context, Error};

/// Steng for bestilling av vafler
#[poise::command(
    prefix_command,
    slash_command,
    rename = "stopp",
    check = "check_is_oracle"
)]
#[tracing::instrument(name = "close", skip(ctx))]
pub async fn close(ctx: Context<'_>) -> Result<(), Error> {
    if !ctx.data().queue.is_open() {
        ctx.say("🔒️ Bestilling er allerede stengt").await?;
        return Ok(());
    }

    ctx.data().queue.close();
    ctx.say("🔒️ Bestilling er nå stengt").await?;

    ctx.serenity_context()
        .set_presence(None, OnlineStatus::Offline);

    Ok(())
}
