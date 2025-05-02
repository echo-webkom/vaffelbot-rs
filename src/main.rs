mod bot;
mod commands;
mod queue;
mod server;

use std::{env, sync::Arc};

use bot::WaffleBot;
use dotenv::dotenv;
use queue::WaffleQueue;
use server::WaffleServer;
use tracing::error;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let redis_url = env::var("REDIS_URL").expect("Expected REDIS_URL in environment");
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let guild_id = env::var("GUILD_ID")
        .expect("Expected GUILD_ID in environment")
        .parse()
        .expect("GUILD_ID must be an integer");

    let redis = redis::Client::open(redis_url).expect("Invalid Redis URL");
    let waffle_queue = WaffleQueue::new(redis);
    let waffle_queue = Arc::new(waffle_queue);

    let waffle_bot = WaffleBot::new(token, guild_id, waffle_queue.clone());
    let waffle_server = WaffleServer::new(waffle_queue.clone());

    let axum_handle = tokio::spawn(async move {
        if let Err(why) = waffle_server.start().await {
            error!("HTTP server error: {why:?}");
        }
    });

    let bot_handle = tokio::spawn(async move {
        if let Err(why) = waffle_bot.start().await {
            error!("Discord client error: {why:?}");
        }
    });

    let _ = tokio::try_join!(axum_handle, bot_handle);
}
