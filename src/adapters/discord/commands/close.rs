use serenity::all::{Mentionable, OnlineStatus, UserId};
use tracing::error;

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
    let guild_id = ctx.guild_id().unwrap().to_string();

    if !ctx.data().queue.is_open(&guild_id) {
        ctx.say("🔒️ Bestilling er allerede stengt").await?;
        return Ok(());
    }

    ctx.data().queue.close(&guild_id).await;

    let mut message = "🔒️ Bestilling er nå stengt".to_string();

    match ctx.data().orders.daily_stats(&guild_id).await {
        Ok(stats) if stats.total_orders > 0 => {
            let vafler = if stats.total_orders == 1 {
                "vaffel"
            } else {
                "vafler"
            };
            message.push_str(&format!(
                "\n\n📊 **Dagens statistikk**\nTotalt stekt: {} {}\n",
                stats.total_orders, vafler
            ));

            if !stats.top_users.is_empty() {
                message.push_str("\n🏆 **Topp bestillere:**\n");
                let medals = ["🥇", "🥈", "🥉"];
                for (i, (user_id, count)) in stats.top_users.iter().enumerate() {
                    if let Ok(id) = user_id.parse::<u64>() {
                        let mention = UserId::new(id).mention();
                        let vafler = if *count == 1 { "vaffel" } else { "vafler" };
                        message.push_str(&format!(
                            "{} {} - {} {}\n",
                            medals[i], mention, count, vafler
                        ));
                    }
                }
            }
        }
        Err(e) => {
            error!("Failed to fetch daily stats: {e}");
        }
        _ => {}
    }

    ctx.say(message).await?;

    ctx.serenity_context()
        .set_presence(None, OnlineStatus::Offline);

    Ok(())
}
