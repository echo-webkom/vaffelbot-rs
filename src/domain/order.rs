#[async_trait::async_trait]
pub trait OrderRepository: Send + Sync {
    async fn record_order(&self, discord_user_id: &str) -> anyhow::Result<()>;
    async fn daily_stats(&self) -> anyhow::Result<DailyStats>;
}

pub struct DailyStats {
    pub total_orders: i64,
    /// (discord_user_id, count)
    pub top_users: Vec<(String, i64)>,
}
