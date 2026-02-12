use std::sync::atomic::{AtomicBool, Ordering};

use redis::AsyncCommands;

use crate::domain::QueueRepository;

const QUEUE_NAME: &str = "queue";

pub struct RedisQueueRepository {
    redis: redis::Client,
    is_open: AtomicBool,
}

impl RedisQueueRepository {
    pub fn new(redis: redis::Client) -> Self {
        Self {
            redis,
            is_open: AtomicBool::new(false),
        }
    }
}

#[async_trait::async_trait]
impl QueueRepository for RedisQueueRepository {
    fn open(&self) {
        self.is_open.store(true, Ordering::Relaxed);
    }

    async fn close(&self) {
        self.is_open.store(false, Ordering::Relaxed);
        self.clear().await;
    }

    fn is_open(&self) -> bool {
        self.is_open.load(Ordering::Relaxed)
    }

    async fn index_of(&self, target: String) -> Option<usize> {
        let mut con = self.redis.get_multiplexed_async_connection().await.ok()?;
        let list: Vec<String> = con.lrange(QUEUE_NAME, 0, -1).await.ok()?;

        list.iter().position(|item| item == &target)
    }

    async fn size(&self) -> usize {
        let mut con = self
            .redis
            .get_multiplexed_async_connection()
            .await
            .ok()
            .unwrap();
        con.llen(QUEUE_NAME).await.unwrap_or(0)
    }

    async fn push(&self, value: String) -> usize {
        let mut con = self
            .redis
            .get_multiplexed_async_connection()
            .await
            .ok()
            .unwrap();
        con.rpush(QUEUE_NAME, value).await.unwrap_or_default()
    }

    async fn pop(&self) -> Option<String> {
        let mut con = self.redis.get_multiplexed_async_connection().await.ok()?;
        let result: redis::RedisResult<Vec<String>> = con.lpop(QUEUE_NAME, None).await;
        result.ok().and_then(|mut vec| vec.pop())
    }

    async fn list(&self) -> Vec<String> {
        let mut con = self
            .redis
            .get_multiplexed_async_connection()
            .await
            .ok()
            .unwrap();
        con.lrange(QUEUE_NAME, 0, -1)
            .await
            .unwrap_or_else(|_| vec![])
    }

    async fn clear(&self) {
        let mut con = self
            .redis
            .get_multiplexed_async_connection()
            .await
            .ok()
            .unwrap();
        let _: () = con.del(QUEUE_NAME).await.unwrap_or_default();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use testcontainers::runners::AsyncRunner;
    use testcontainers_modules::redis::Redis;
    use tokio::sync::OnceCell;

    struct TestRedis {
        _node: testcontainers::ContainerAsync<Redis>,
        client: redis::Client,
    }

    static REDIS: OnceCell<TestRedis> = OnceCell::const_new();

    async fn init_redis() -> &'static TestRedis {
        REDIS
            .get_or_init(|| async {
                if std::env::var("DOCKER_HOST").is_err() {
                    let home = std::env::var("HOME").unwrap();
                    let socket = format!("{home}/.colima/default/docker.sock");
                    if std::path::Path::new(&socket).exists() {
                        std::env::set_var("DOCKER_HOST", format!("unix://{socket}"));
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
        queue.clear().await;
        queue
    }

    #[tokio::test]
    async fn test_push_and_pop() {
        let queue = setup().await;

        queue.push("foo".to_string()).await;
        queue.push("bar".to_string()).await;

        assert_eq!(queue.size().await, 2);
        assert_eq!(queue.index_of("bar".to_string()).await, Some(1));

        let popped = queue.pop().await;
        assert_eq!(popped, Some("foo".to_string()));
        assert_eq!(queue.size().await, 1);

        let remaining = queue.list().await;
        assert_eq!(remaining, vec!["bar".to_string()]);
    }

    #[tokio::test]
    async fn test_list() {
        let queue = setup().await;

        queue.push("foo".to_string()).await;
        queue.push("bar".to_string()).await;

        let list = queue.list().await;
        assert_eq!(list, vec!["foo".to_string(), "bar".to_string()]);
    }

    #[tokio::test]
    async fn test_index_of() {
        let queue = setup().await;

        queue.push("foo".to_string()).await;
        queue.push("bar".to_string()).await;

        assert_eq!(queue.index_of("foo".to_string()).await, Some(0));
        assert_eq!(queue.index_of("bar".to_string()).await, Some(1));
        assert_eq!(queue.index_of("baz".to_string()).await, None);
    }

    #[tokio::test]
    async fn test_size() {
        let queue = setup().await;

        assert_eq!(queue.size().await, 0);

        queue.push("foo".to_string()).await;
        queue.push("bar".to_string()).await;

        assert_eq!(queue.size().await, 2);
    }

    #[tokio::test]
    async fn test_clear() {
        let queue = setup().await;

        queue.push("foo".to_string()).await;
        queue.push("bar".to_string()).await;

        assert_eq!(queue.size().await, 2);

        queue.clear().await;

        assert_eq!(queue.size().await, 0);
    }
}
