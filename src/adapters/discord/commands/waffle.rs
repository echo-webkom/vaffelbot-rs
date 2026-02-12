use crate::adapters::discord::{Context, Error};
use crate::domain::QueueEntry;

/// Få en orakel til å steke vaffel til deg
#[poise::command(prefix_command, slash_command, rename = "vaffel")]
#[tracing::instrument(name = "waffle", skip(ctx))]
pub async fn waffle(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap().to_string();

    if !ctx.data().queue.is_open(&guild_id) {
        ctx.say("🏮 Bestilling er stengt").await?;
        return Ok(());
    }

    let user_id = ctx.author().id.to_string();
    let display_name = ctx.author().name.clone();

    let message = match ctx.data().queue.index_of(&guild_id, &user_id).await {
        Some(index) => format!(
            "⏲️ Du er **allerede** i køen. Du er nummer **{}** i køen.",
            index + 1
        ),
        None => {
            let size = ctx.data().queue.size(&guild_id).await;
            let entry = QueueEntry::new(user_id, display_name);
            ctx.data().queue.push(&guild_id, entry).await;
            format!("⏲️ Du er nå i køen. Du er nummer **{}** i køen.", size + 1)
        }
    };

    ctx.say(message).await?;

    Ok(())
}
