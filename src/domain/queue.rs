pub trait QueueRepository: Send + Sync {
    /// Open the queue to allow new entries
    fn open(&self);

    /// Close the queue to prevent new entries
    fn close(&self);

    /// Check if the queue is currently open
    fn is_open(&self) -> bool;

    /// Find the position of an item in the queue
    /// Returns None if the item is not found
    fn index_of(&self, target: String) -> Option<usize>;

    /// Get the current size of the queue
    fn size(&self) -> usize;

    /// Add an item to the end of the queue
    /// Returns the new size of the queue
    fn push(&self, value: String) -> usize;

    /// Remove and return the item at the front of the queue
    /// Returns None if the queue is empty
    fn pop(&self) -> Option<String>;

    /// Get all items in the queue without removing them
    fn list(&self) -> Vec<String>;
}
