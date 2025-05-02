use std::{
    num::NonZero,
    sync::atomic::{AtomicBool, Ordering},
};

use redis::Commands;

const QUEUE_NAME: &str = "queue";

pub struct WaffleQueue {
    redis: redis::Client,
    is_open: AtomicBool,
}

impl WaffleQueue {
    pub fn new(redis: redis::Client) -> Self {
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
        let mut con = match self.redis.get_connection() {
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
        let mut con = self.redis.get_connection().unwrap();
        con.llen(QUEUE_NAME).unwrap_or(0) as usize
    }

    pub fn push(&self, value: String) -> usize {
        let mut con = self.redis.get_connection().unwrap();

        match con.rpush(QUEUE_NAME, value) {
            Ok(len) => len,
            Err(_) => 0,
        }
    }

    pub fn pop(&self) -> Option<String> {
        let mut con = self.redis.get_connection().unwrap();
        con.lpop(QUEUE_NAME, Some(NonZero::new(1).unwrap()))
            .ok()
            .flatten()
    }

    pub fn drain(&self) -> Vec<String> {
        let mut con = self.redis.get_connection().unwrap();
        let mut drained = Vec::new();

        loop {
            let item = con.lpop(QUEUE_NAME, None);
            match item {
                Ok(val) => drained.push(val),
                Err(_) => break,
            }
        }

        drained
    }
}
