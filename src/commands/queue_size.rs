use crate::bot::{Context, Error};

/// Se hvor mange som er foran deg i køen
#[poise::command(prefix_command, slash_command, rename = "kø")]
#[tracing::instrument(name = "queue", skip(ctx))]
pub async fn queue(ctx: Context<'_>) -> Result<(), Error> {
    if !ctx.data().queue.is_open() {
        ctx.say("Bestilling er stengt").await?;
        return Ok(());
    }

    let user_id = ctx.author().id.to_string();
    let message = match ctx.data().queue.index_of(user_id) {
        Some(index) => format!("Du er {} i køen", index + 1),
        None => "Du er ikke i køen".to_string(),
    };

    ctx.say(message).await?;

    Ok(())
}
