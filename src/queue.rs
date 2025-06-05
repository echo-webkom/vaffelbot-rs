use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};

use redis::Commands;

const QUEUE_NAME: &str = "queue";

pub struct WaffleQueue {
    redis: Arc<Mutex<redis::Client>>,
    is_open: AtomicBool,
}

impl WaffleQueue {
    pub fn new(redis: Arc<Mutex<redis::Client>>) -> Self {
        Self {
            redis,
            is_open: AtomicBool::new(false),
        }
    }

    pub fn open(&self) {
        self.is_open.store(true, Ordering::Relaxed);
    }

    pub fn close(&self) {
        self.is_open.store(false, Ordering::Relaxed);
    }

    pub fn is_open(&self) -> bool {
        self.is_open.load(Ordering::Relaxed)
    }

    pub fn index_of(&self, target: String) -> Option<usize> {
        let mut con = match self.redis.lock().unwrap().get_connection() {
            Ok(c) => c,
            Err(_) => return None,
        };

        let list: Vec<String> = match con.lrange(QUEUE_NAME, 0, -1) {
            Ok(l) => l,
            Err(_) => return None,
        };

        list.iter().position(|item| item == &target)
    }

    pub fn size(&self) -> usize {
        let mut con = self.redis.lock().unwrap().get_connection().unwrap();
        con.llen(QUEUE_NAME).unwrap_or(0) as usize
    }

    pub fn push(&self, value: String) -> usize {
        let mut con = self.redis.lock().unwrap().get_connection().unwrap();

        con.rpush(QUEUE_NAME, value).unwrap_or_default()
    }

    pub fn pop(&self) -> Option<String> {
        let mut con = self.redis.lock().unwrap().get_connection().unwrap();
        let result: redis::RedisResult<Vec<String>> = con.lpop(QUEUE_NAME, None);
        result.ok().and_then(|mut vec| vec.pop())
    }

    pub fn list(&self) -> Vec<String> {
        let mut con = self.redis.lock().unwrap().get_connection().unwrap();
        con.lrange(QUEUE_NAME, 0, -1).unwrap_or_else(|_| vec![])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use testcontainers::runners::SyncRunner;
    use testcontainers_modules::redis::Redis;

    #[test]
    fn test_push_and_pop() {
        let node = Redis::default().start().unwrap();
        let host_ip = node.get_host().unwrap();
        let host_port = node.get_host_port_ipv4(6379).unwrap();
        let url = format!("redis://{host_ip}:{host_port}");
        let client = Arc::new(Mutex::new(redis::Client::open(url).unwrap()));
        let queue = WaffleQueue::new(client);

        queue.push("foo".to_string());
        queue.push("bar".to_string());

        assert_eq!(queue.size(), 2);
        assert_eq!(queue.index_of("bar".to_string()), Some(1));

        let popped = queue.pop();
        assert_eq!(popped, Some("foo".to_string()));
        assert_eq!(queue.size(), 1);

        let remaining = queue.list();
        assert_eq!(remaining, vec!["bar".to_string()]);
    }

    #[test]
    fn test_list() {
        let node = Redis::default().start().unwrap();
        let host_ip = node.get_host().unwrap();
        let host_port = node.get_host_port_ipv4(6379).unwrap();
        let url = format!("redis://{host_ip}:{host_port}");
        let client = Arc::new(Mutex::new(redis::Client::open(url).unwrap()));
        let queue = WaffleQueue::new(client);

        queue.push("foo".to_string());
        queue.push("bar".to_string());

        let list = queue.list();
        assert_eq!(list, vec!["foo".to_string(), "bar".to_string()]);
    }

    #[test]
    fn test_index_of() {
        let node = Redis::default().start().unwrap();
        let host_ip = node.get_host().unwrap();
        let host_port = node.get_host_port_ipv4(6379).unwrap();
        let url = format!("redis://{host_ip}:{host_port}");
        let client = Arc::new(Mutex::new(redis::Client::open(url).unwrap()));
        let queue = WaffleQueue::new(client);

        queue.push("foo".to_string());
        queue.push("bar".to_string());

        assert_eq!(queue.index_of("foo".to_string()), Some(0));
        assert_eq!(queue.index_of("bar".to_string()), Some(1));
        assert_eq!(queue.index_of("baz".to_string()), None);
    }

    #[test]
    fn test_size() {
        let node = Redis::default().start().unwrap();
        let host_ip = node.get_host().unwrap();
        let host_port = node.get_host_port_ipv4(6379).unwrap();
        let url = format!("redis://{host_ip}:{host_port}");
        let client = Arc::new(Mutex::new(redis::Client::open(url).unwrap()));
        let queue = WaffleQueue::new(client);

        assert_eq!(queue.size(), 0);

        queue.push("foo".to_string());
        queue.push("bar".to_string());

        assert_eq!(queue.size(), 2);
    }
}
