#[async_trait::async_trait]
pub trait OrderRepository: Send + Sync {
    async fn record_order(&self, discord_user_id: &str) -> anyhow::Result<()>;
}
