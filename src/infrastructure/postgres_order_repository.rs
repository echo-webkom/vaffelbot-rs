use sqlx::PgPool;
use tracing::{debug, error, info, instrument};

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
    #[instrument(skip(self), fields(discord_user_id, guild_id))]
    async fn record_order(&self, discord_user_id: &str, guild_id: &str) -> anyhow::Result<()> {
        debug!(discord_user_id, guild_id, "Recording order");
        sqlx::query!(
            "INSERT INTO orders (discord_user_id, guild_id) VALUES ($1, $2)",
            discord_user_id,
            guild_id
        )
        .execute(&self.pool)
        .await
        .map_err(|e| {
            error!(discord_user_id, guild_id, error = ?e, "Failed to record order in database");
            e
        })?;
        info!(discord_user_id, guild_id, "Order recorded successfully");
        Ok(())
    }

    #[instrument(skip(self), fields(guild_id))]
    async fn daily_stats(&self, guild_id: &str) -> anyhow::Result<DailyStats> {
        debug!(guild_id, "Fetching daily stats");

        let total = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM orders WHERE fulfilled_at::date = CURRENT_DATE AND guild_id = $1",
            guild_id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            error!(guild_id, error = ?e, "Failed to fetch total order count");
            e
        })?
        .unwrap_or(0);

        let top_users: Vec<(String, i64)> = sqlx::query!(
            "SELECT discord_user_id, COUNT(*) as count FROM orders \
             WHERE fulfilled_at::date = CURRENT_DATE AND guild_id = $1 \
             GROUP BY discord_user_id \
             ORDER BY count DESC \
             LIMIT 3",
            guild_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| {
            error!(guild_id, error = ?e, "Failed to fetch top users");
            e
        })?
        .into_iter()
        .map(|row| (row.discord_user_id, row.count.unwrap_or(0)))
        .collect();

        info!(
            guild_id,
            total_orders = total,
            top_users_count = top_users.len(),
            "Retrieved daily stats"
        );

        Ok(DailyStats {
            total_orders: total,
            top_users,
        })
    }
}
