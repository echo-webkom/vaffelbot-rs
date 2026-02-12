use crate::adapters::discord::{Context, Error};

/// Se hvor mange som er foran deg i køen
#[tracing::instrument(name = "queue", skip(ctx))]
#[poise::command(prefix_command, slash_command, rename = "kø")]
pub async fn queue(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap().to_string();

    if !ctx.data().queue.is_open(&guild_id) {
        ctx.say("🚨 Bestilling er stengt").await?;
        return Ok(());
    }

    let user_id = ctx.author().id.to_string();
    let message = match ctx.data().queue.index_of(&guild_id, &user_id).await {
        Some(index) => format!("😎 Du er {} i køen", index + 1),
        None => "🚨 Du er ikke i køen.".to_string(),
    };

    ctx.say(message).await?;

    Ok(())
}
