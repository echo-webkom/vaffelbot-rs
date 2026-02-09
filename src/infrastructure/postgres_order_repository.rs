use sqlx::PgPool;

use crate::domain::OrderRepository;

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
}
