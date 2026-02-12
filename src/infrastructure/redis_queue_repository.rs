use std::{collections::HashSet, sync::RwLock};

use redis::AsyncCommands;

use crate::domain::{QueueEntry, QueueRepository};

fn queue_key(guild_id: &str) -> String {
    format!("queue:{guild_id}")
}

pub struct RedisQueueRepository {
    redis: redis::Client,
    open_guilds: RwLock<HashSet<String>>,
}

impl RedisQueueRepository {
    pub fn new(redis: redis::Client) -> Self {
        Self {
            redis,
            open_guilds: RwLock::new(HashSet::new()),
        }
    }
}

#[async_trait::async_trait]
impl QueueRepository for RedisQueueRepository {
    fn open(&self, guild_id: &str) {
        self.open_guilds
            .write()
            .unwrap()
            .insert(guild_id.to_string());
    }

    async fn close(&self, guild_id: &str) {
        self.open_guilds.write().unwrap().remove(guild_id);
        self.clear(guild_id).await;
    }

    fn is_open(&self, guild_id: &str) -> bool {
        self.open_guilds.read().unwrap().contains(guild_id)
    }

    async fn index_of(&self, guild_id: &str, user_id: &str) -> Option<usize> {
        let key = queue_key(guild_id);
        let mut con = self.redis.get_multiplexed_async_connection().await.ok()?;
        let list: Vec<String> = con.lrange(&key, 0, -1).await.ok()?;

        list.iter().position(|json_str| {
            serde_json::from_str::<QueueEntry>(json_str)
                .map(|entry| entry.user_id == user_id)
                .unwrap_or(false)
        })
    }

    async fn size(&self, guild_id: &str) -> usize {
        let key = queue_key(guild_id);
        let mut con = match self.redis.get_multiplexed_async_connection().await {
            Ok(con) => con,
            Err(_) => return 0,
        };
        con.llen(&key).await.unwrap_or(0)
    }

    async fn push(&self, guild_id: &str, entry: QueueEntry) -> usize {
        let key = queue_key(guild_id);
        let json = serde_json::to_string(&entry).unwrap();
        let mut con = match self.redis.get_multiplexed_async_connection().await {
            Ok(con) => con,
            Err(_) => return 0,
        };
        con.rpush(&key, json).await.unwrap_or_default()
    }

    async fn pop(&self, guild_id: &str) -> Option<QueueEntry> {
        let key = queue_key(guild_id);
        let mut con = self.redis.get_multiplexed_async_connection().await.ok()?;
        let result: redis::RedisResult<Vec<String>> = con.lpop(&key, None).await;
        result
            .ok()
            .and_then(|mut vec| vec.pop())
            .and_then(|json_str| serde_json::from_str(&json_str).ok())
    }

    async fn list(&self, guild_id: &str) -> Vec<QueueEntry> {
        let key = queue_key(guild_id);
        let mut con = match self.redis.get_multiplexed_async_connection().await {
            Ok(con) => con,
            Err(_) => return vec![],
        };
        let json_list: Vec<String> = con.lrange(&key, 0, -1).await.unwrap_or_else(|_| vec![]);
        json_list
            .into_iter()
            .filter_map(|json_str| serde_json::from_str(&json_str).ok())
            .collect()
    }

    async fn clear(&self, guild_id: &str) {
        let key = queue_key(guild_id);
        if let Ok(mut con) = self.redis.get_multiplexed_async_connection().await {
            let _: () = con.del(&key).await.unwrap_or_default();
        }
    }
}

#[cfg(test)]
mod tests {
    use std::env::home_dir;

    use super::*;

    use testcontainers::runners::AsyncRunner;
    use testcontainers_modules::redis::Redis;
    use tokio::sync::OnceCell;

    const TEST_GUILD: &str = "test-guild";

    struct TestRedis {
        _node: testcontainers::ContainerAsync<Redis>,
        client: redis::Client,
    }

    static REDIS: OnceCell<TestRedis> = OnceCell::const_new();

    async fn init_redis() -> &'static TestRedis {
        REDIS
            .get_or_init(|| async {
                if std::env::var("DOCKER_HOST").is_err() {
                    let socket = home_dir()
                        .expect("Failed to get home directory")
                        .join(".colima/default/docker.sock");
                    if std::path::Path::new(&socket).exists() {
                        std::env::set_var(
                            "DOCKER_HOST",
                            format!("unix://{}", socket.to_string_lossy()),
                        );
                    }
                }

                let node = Redis::default().start().await.unwrap();
                let host_ip = node.get_host().await.unwrap();
                let host_port = node.get_host_port_ipv4(6379).await.unwrap();
                let url = format!("redis://{host_ip}:{host_port}");
                let client = redis::Client::open(url).unwrap();
                TestRedis {
                    _node: node,
                    client,
                }
            })
            .await
    }

    async fn setup() -> RedisQueueRepository {
        let redis = init_redis().await;
        let queue = RedisQueueRepository::new(redis.client.clone());
        queue.clear(TEST_GUILD).await;
        queue
    }

    #[tokio::test]
    async fn test_push_and_pop() {
        let queue = setup().await;
        let guild = "test-push-and-pop";
        queue.clear(guild).await;

        let foo = QueueEntry::new("foo".to_string(), "Foo User".to_string());
        let bar = QueueEntry::new("bar".to_string(), "Bar User".to_string());

        queue.push(guild, foo.clone()).await;
        queue.push(guild, bar.clone()).await;

        assert_eq!(queue.size(guild).await, 2);
        assert_eq!(queue.index_of(guild, "bar").await, Some(1));

        let popped = queue.pop(guild).await;
        assert_eq!(popped, Some(foo));
        assert_eq!(queue.size(guild).await, 1);

        let remaining = queue.list(guild).await;
        assert_eq!(remaining, vec![bar]);
    }

    #[tokio::test]
    async fn test_list() {
        let queue = setup().await;
        let guild = "test-list";
        queue.clear(guild).await;

        let foo = QueueEntry::new("foo".to_string(), "Foo User".to_string());
        let bar = QueueEntry::new("bar".to_string(), "Bar User".to_string());

        queue.push(guild, foo.clone()).await;
        queue.push(guild, bar.clone()).await;

        let list = queue.list(guild).await;
        assert_eq!(list, vec![foo, bar]);
    }

    #[tokio::test]
    async fn test_index_of() {
        let queue = setup().await;
        let guild = "test-index-of";
        queue.clear(guild).await;

        let foo = QueueEntry::new("foo".to_string(), "Foo User".to_string());
        let bar = QueueEntry::new("bar".to_string(), "Bar User".to_string());

        queue.push(guild, foo).await;
        queue.push(guild, bar).await;

        assert_eq!(queue.index_of(guild, "foo").await, Some(0));
        assert_eq!(queue.index_of(guild, "bar").await, Some(1));
        assert_eq!(queue.index_of(guild, "baz").await, None);
    }

    #[tokio::test]
    async fn test_size() {
        let queue = setup().await;
        let guild = "test-size";
        queue.clear(guild).await;

        assert_eq!(queue.size(guild).await, 0);

        let foo = QueueEntry::new("foo".to_string(), "Foo User".to_string());
        let bar = QueueEntry::new("bar".to_string(), "Bar User".to_string());

        queue.push(guild, foo).await;
        queue.push(guild, bar).await;

        assert_eq!(queue.size(guild).await, 2);
    }

    #[tokio::test]
    async fn test_clear() {
        let queue = setup().await;
        let guild = "test-clear";
        queue.clear(guild).await;

        let foo = QueueEntry::new("foo".to_string(), "Foo User".to_string());
        let bar = QueueEntry::new("bar".to_string(), "Bar User".to_string());

        queue.push(guild, foo).await;
        queue.push(guild, bar).await;

        assert_eq!(queue.size(guild).await, 2);

        queue.clear(guild).await;

        assert_eq!(queue.size(guild).await, 0);
    }
}
