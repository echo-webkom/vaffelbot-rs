#[async_trait::async_trait]
pub trait QueueRepository: Send + Sync {
    /// Open the queue to allow new entries
    fn open(&self, guild_id: &str);

    /// Close the queue to prevent new entries
    async fn close(&self, guild_id: &str);

    /// Check if the queue is currently open
    fn is_open(&self, guild_id: &str) -> bool;

    /// Find the position of an item in the queue
    /// Returns None if the item is not found
    async fn index_of(&self, guild_id: &str, target: String) -> Option<usize>;

    /// Get the current size of the queue
    async fn size(&self, guild_id: &str) -> usize;

    /// Add an item to the end of the queue
    /// Returns the new size of the queue
    async fn push(&self, guild_id: &str, value: String) -> usize;

    /// Remove the item at the front of the queue and return it
    /// Returns None if the queue is empty
    async fn pop(&self, guild_id: &str) -> Option<String>;

    /// Get all items in the queue
    async fn list(&self, guild_id: &str) -> Vec<String>;

    /// Clear the queue
    async fn clear(&self, guild_id: &str);
}
