//! A priority queue with a maximum size that evicts lowest-priority items.

use std::hash::Hash;

use priority_queue::DoublePriorityQueue;

/// A priority queue that maintains a maximum size by evicting lowest-priority items.
pub struct BoundedQueue<I: Hash + Eq, P: Ord> {
    queue: DoublePriorityQueue<I, P>,
    max_size: usize,
}

impl<I: Hash + Eq, P: Ord> BoundedQueue<I, P> {
    pub fn new(max_size: usize) -> Self {
        Self {
            queue: DoublePriorityQueue::with_capacity(max_size),
            max_size,
        }
    }

    /// Push an item. If over capacity, evicts lowest-priority items.
    /// No-op if at capacity and priority <= current minimum.
    pub fn push(&mut self, item: I, priority: P) {
        if self.queue.len() >= self.max_size {
            if let Some((_, min_p)) = self.queue.peek_min() {
                if priority <= *min_p {
                    return; // would be immediately evicted
                }
            }
            // Better than min: pop min first, then push (avoids temporary overcapacity)
            self.queue.pop_min();
        }
        self.queue.push(item, priority);
    }

    /// Pop the highest-priority item.
    pub fn pop(&mut self) -> Option<(I, P)> {
        self.queue.pop_max()
    }

    /// Peek at the highest-priority item without removing it.
    pub fn peek(&self) -> Option<(&I, &P)> {
        self.queue.peek_max()
    }

    /// Change the maximum size. Evicts if new size is smaller.
    pub fn set_max_size(&mut self, new_max: usize) {
        self.max_size = new_max;
        self.trim();
    }

    /// Evict lowest-priority items until at or below max_size.
    fn trim(&mut self) {
        while self.queue.len() > self.max_size {
            self.queue.pop_min();
        }
    }

    pub fn len(&self) -> usize {
        self.queue.len()
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_creates_empty_queue() {
        let q: BoundedQueue<i32, i32> = BoundedQueue::new(5);
        assert!(q.is_empty());
        assert_eq!(q.len(), 0);
    }

    #[test]
    fn push_adds_items() {
        let mut q = BoundedQueue::new(5);
        q.push("a", 1);
        q.push("b", 2);
        assert_eq!(q.len(), 2);
    }

    #[test]
    fn pop_returns_highest_priority() {
        let mut q = BoundedQueue::new(5);
        q.push("low", 1);
        q.push("high", 10);
        q.push("mid", 5);

        assert_eq!(q.pop(), Some(("high", 10)));
        assert_eq!(q.pop(), Some(("mid", 5)));
        assert_eq!(q.pop(), Some(("low", 1)));
        assert_eq!(q.pop(), None);
    }

    #[test]
    fn peek_shows_highest_without_removing() {
        let mut q = BoundedQueue::new(5);
        q.push("low", 1);
        q.push("high", 10);

        assert_eq!(q.peek(), Some((&"high", &10)));
        assert_eq!(q.len(), 2);
    }

    #[test]
    fn evicts_lowest_when_over_capacity() {
        let mut q = BoundedQueue::new(3);
        q.push("a", 1);
        q.push("b", 2);
        q.push("c", 3);
        q.push("d", 4); // should evict "a" (priority 1)

        assert_eq!(q.len(), 3);
        // "a" should be gone
        assert_eq!(q.pop(), Some(("d", 4)));
        assert_eq!(q.pop(), Some(("c", 3)));
        assert_eq!(q.pop(), Some(("b", 2)));
        assert_eq!(q.pop(), None);
    }

    #[test]
    fn evicts_new_item_if_lower_than_all() {
        let mut q = BoundedQueue::new(2);
        q.push("high", 10);
        q.push("mid", 5);
        q.push("low", 1); // should be immediately evicted

        assert_eq!(q.len(), 2);
        assert_eq!(q.pop(), Some(("high", 10)));
        assert_eq!(q.pop(), Some(("mid", 5)));
    }

    #[test]
    fn set_max_size_shrinks_queue() {
        let mut q = BoundedQueue::new(5);
        q.push("a", 1);
        q.push("b", 2);
        q.push("c", 3);
        q.push("d", 4);
        q.push("e", 5);

        q.set_max_size(2);
        assert_eq!(q.len(), 2);
        assert_eq!(q.pop(), Some(("e", 5)));
        assert_eq!(q.pop(), Some(("d", 4)));
    }

    #[test]
    fn set_max_size_allows_growth() {
        let mut q = BoundedQueue::new(2);
        q.push("a", 1);
        q.push("b", 2);

        q.set_max_size(5);
        q.push("c", 3);
        q.push("d", 4);
        q.push("e", 5);

        assert_eq!(q.len(), 5);
    }
}
