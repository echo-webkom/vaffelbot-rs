use std::sync::Arc;

use r2d2::Pool;
use tracing::error;

use crate::{
    adapters::{DiscordAdapter, HttpAdapter},
    config::Config,
    infrastructure::RedisQueueRepository,
};

pub mod adapters;
pub mod config;
pub mod domain;
pub mod infrastructure;

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

        let queue: Arc<dyn domain::QueueRepository> =
            Arc::new(RedisQueueRepository::new(redis_pool));

        let discord_adapter = DiscordAdapter::new(self.config.discord_token.clone(), queue.clone());
        let http_adapter = HttpAdapter::new(queue.clone());

        let axum_handle = tokio::spawn(async move {
            if let Err(why) = http_adapter.start().await {
                error!("HTTP server error: {why:?}");
            }
        });

        let bot_handle = tokio::spawn(async move {
            if let Err(why) = discord_adapter.start().await {
                error!("Discord bot error: {why:?}");
            }
        });

        let _ = tokio::try_join!(axum_handle, bot_handle);

        Ok(())
    }
}
