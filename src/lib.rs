use std::sync::Arc;

use r2d2::Pool;
use tracing::error;

use crate::{bot::Bot, config::Config, queue::Queue, server::Server};

pub mod bot;
mod commands;
pub mod config;
pub mod queue;
pub mod server;

pub struct VaffelBot {
    config: Config,
}

impl VaffelBot {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub async fn run(self) -> anyhow::Result<()> {
        let redis = redis::Client::open(self.config.redis_url.clone()).expect("Invalid Redis URL");
        let redis_pool = Pool::builder()
            .build(redis)
            .expect("Failed to create Redis connection pool");

        let queue = Arc::new(Queue::new(redis_pool));

        let bot = Bot::new(self.config.discord_token.clone(), queue.clone());
        let server = Server::new(queue.clone());

        let axum_handle = tokio::spawn(async move {
            if let Err(why) = server.start().await {
                error!("HTTP server error: {why:?}");
            }
        });

        let bot_handle = tokio::spawn(async move {
            if let Err(why) = bot.start().await {
                error!("Discord bot error: {why:?}");
            }
        });

        let _ = tokio::try_join!(axum_handle, bot_handle);

        Ok(())
    }
}
