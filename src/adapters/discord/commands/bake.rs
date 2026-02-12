use serenity::all::{MessageBuilder, UserId};
use tracing::error;

use crate::adapters::discord::{check_is_oracle, Context, Error};
use crate::domain::QueueEntry;

/// Stek vaffel
#[tracing::instrument(name = "bake", skip(ctx))]
#[poise::command(
    prefix_command,
    slash_command,
    rename = "stekt",
    check = "check_is_oracle"
)]
pub async fn bake(
    ctx: Context<'_>,
    #[description = "Hvor mange vafler?"] amount: usize,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap().to_string();

    if !ctx.data().queue.is_open(&guild_id) {
        ctx.say("🔒️ Bestilling er stengt").await?;
        return Ok(());
    }

    let mut baked: Vec<QueueEntry> = vec![];
    let n = ctx.data().queue.size(&guild_id).await.min(amount);

    for _ in 0..n {
        if let Some(entry) = ctx.data().queue.pop(&guild_id).await {
            baked.push(entry);
        } else {
            break;
        }
    }

    let message = if baked.is_empty() {
        "😟 Ingen å steke vafler til.".to_string()
    } else {
        let mut msg = MessageBuilder::new();
        msg.push("🧇 Stekte ").push(baked.len().to_string());

        if baked.len() == 1 {
            msg.push(" en vaffel til: ");
            let user_id = UserId::new(baked[0].user_id.parse::<u64>().unwrap());
            msg.mention(&user_id);
        } else {
            msg.push(" vafler til: ");

            for (i, entry) in baked.iter().enumerate() {
                let user_id = UserId::new(entry.user_id.parse::<u64>().unwrap());

                if i == baked.len() - 1 {
                    msg.push(" og ").mention(&user_id);
                } else {
                    msg.mention(&user_id).push(", ");
                }
            }
        }

        msg.build()
    };

    for entry in &baked {
        if let Err(e) = ctx
            .data()
            .orders
            .record_order(&entry.user_id, &guild_id)
            .await
        {
            error!(
                user_id = %entry.user_id,
                guild_id = %guild_id,
                error = ?e,
                "Failed to record order"
            );
        }
    }

    ctx.say(message).await?;

    Ok(())
}
