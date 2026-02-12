use crate::adapters::discord::{Context, Error};

/// Få en orakel til å steke vaffel til deg
#[poise::command(prefix_command, slash_command, rename = "vaffel")]
#[tracing::instrument(name = "waffle", skip(ctx))]
pub async fn waffle(ctx: Context<'_>) -> Result<(), Error> {
    if !ctx.data().queue.is_open() {
        ctx.say("🏮 Bestilling er stengt").await?;
        return Ok(());
    }

    let user_id = ctx.author().id.to_string();
    let message = match ctx.data().queue.index_of(user_id.clone()).await {
        Some(index) => format!(
            "⏲️ Du er **allerede** i køen. Det er **{} foran deg**.",
            index
        ),
        None => {
            let size = ctx.data().queue.size().await;
            ctx.data().queue.push(user_id).await;
            format!("⏲️ Du er nå i køen. Det er **{} foran deg**.", size)
        }
    };

    ctx.say(message).await?;

    Ok(())
}
