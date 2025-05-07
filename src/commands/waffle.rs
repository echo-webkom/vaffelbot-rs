use crate::bot::{Context, Error};

/// Få en orakel til å steke vaffel til deg
#[poise::command(prefix_command, slash_command, rename = "vaffel")]
#[tracing::instrument(name = "waffle", skip(ctx))]
pub async fn waffle(ctx: Context<'_>) -> Result<(), Error> {
    if !ctx.data().queue.is_open() {
        ctx.say("Bestilling er stengt").await?;
        return Ok(());
    }

    let user_id = ctx.author().id.to_string();
    let message = match ctx.data().queue.index_of(user_id.clone()) {
        Some(index) => format!("Du er {} i køen", index + 1),
        None => {
            let size = ctx.data().queue.size();
            ctx.data().queue.push(user_id);
            format!("Du er nå i køen. Det er {} personer foran deg", size)
        }
    };

    ctx.say(message).await?;

    Ok(())
}
