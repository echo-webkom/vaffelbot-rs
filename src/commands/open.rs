use serenity::all::{ActivityData, OnlineStatus};

use crate::bot::{check_is_oracle, Context, Error};

/// Åpne for bestilling av vafler
#[poise::command(
    prefix_command,
    slash_command,
    rename = "start",
    check = "check_is_oracle"
)]
#[tracing::instrument(name = "open", skip(ctx))]
pub async fn open(ctx: Context<'_>) -> Result<(), Error> {
    if ctx.data().queue.is_open() {
        ctx.say("Bestilling er allerede åpnet").await?;
        return Ok(());
    }

    ctx.data().queue.open();

    ctx.serenity_context().set_presence(
        Some(ActivityData::playing("🧇 Lager vafler")),
        OnlineStatus::Offline,
    );

    Ok(())
}
