use sqlx::PgPool;

use crate::domain::{DailyStats, OrderRepository};

pub struct PostgresOrderRepository {
    pool: PgPool,
}

impl PostgresOrderRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl OrderRepository for PostgresOrderRepository {
    async fn record_order(&self, discord_user_id: &str) -> anyhow::Result<()> {
        sqlx::query!(
            "INSERT INTO orders (discord_user_id) VALUES ($1)",
            discord_user_id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn daily_stats(&self) -> anyhow::Result<DailyStats> {
        let total = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM orders WHERE fulfilled_at::date = CURRENT_DATE"
        )
        .fetch_one(&self.pool)
        .await?
        .unwrap_or(0);

        let top_users = sqlx::query!(
            "SELECT discord_user_id, COUNT(*) as count FROM orders \
             WHERE fulfilled_at::date = CURRENT_DATE \
             GROUP BY discord_user_id \
             ORDER BY count DESC \
             LIMIT 3"
        )
        .fetch_all(&self.pool)
        .await?
        .into_iter()
        .map(|row| (row.discord_user_id, row.count.unwrap_or(0)))
        .collect();

        Ok(DailyStats {
            total_orders: total,
            top_users,
        })
    }
}
