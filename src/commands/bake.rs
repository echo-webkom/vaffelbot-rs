use serenity::all::{MessageBuilder, UserId};
use tracing::debug;

use crate::bot::{check_is_oracle, Context, Error};

/// Stek vaffel
#[poise::command(
    prefix_command,
    slash_command,
    rename = "stek",
    check = "check_is_oracle"
)]
#[tracing::instrument(name = "bake", skip(ctx))]
pub async fn bake(
    ctx: Context<'_>,
    #[description = "Hvor mange vafler?"] amount: usize,
) -> Result<(), Error> {
    debug!("bake command called");

    if !ctx.data().queue.is_open() {
        ctx.say("🔒️ Bestilling er stengt").await?;
        return Ok(());
    }

    let mut baked = vec![];
    let n = ctx.data().queue.size().min(amount);

    for _ in 0..n {
        if let Some(user_id) = ctx.data().queue.pop() {
            baked.push(user_id);
        } else {
            break;
        }
    }

    let message = if baked.is_empty() {
        let mut s = "😟 Ingen å steke vafler til.".to_string();
        if amount > n {
            s.push_str(format!(" ({} vafler til overs)", amount - n).as_str());
        }
        s
    } else {
        let mut msg = MessageBuilder::new();
        msg.push("🧇 Stekte ").push(baked.len().to_string());

        if baked.len() == 1 {
            msg.push(" en vaffel til: ");
            let user_id = UserId::new(baked[0].parse::<u64>().unwrap());
            msg.mention(&user_id);
        } else {
            msg.push(" vafler til: ");

            for (i, user_id) in baked.iter().enumerate() {
                let user_id = UserId::new(user_id.parse::<u64>().unwrap());

                if i == baked.len() - 1 {
                    msg.push(" og ").mention(&user_id);
                } else {
                    msg.mention(&user_id).push(", ");
                }
            }
        }

        if amount > n {
            msg.push(format!(" ({} vafler til overs)", amount - n));
        }

        msg.build()
    };

    ctx.say(message).await?;

    Ok(())
}
