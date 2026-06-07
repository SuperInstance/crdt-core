//! Positive-Negative Counter (PN-Counter): supports both increment and decrement.

use crate::gcounter::GCounter;

/// A PN-Counter CRDT built from two G-Counters (positive and negative).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PNCounter {
    positive: GCounter,
    negative: GCounter,
}

impl PNCounter {
    pub fn new() -> Self {
        Self { positive: GCounter::new(), negative: GCounter::new() }
    }

    /// Increment the counter for a node.
    pub fn increment(&mut self, node_id: u64) {
        self.positive.increment(node_id);
    }

    /// Decrement the counter for a node.
    pub fn decrement(&mut self, node_id: u64) {
        self.negative.increment(node_id);
    }

    /// Get the current value (positive - negative).
    pub fn value(&self) -> i64 {
        self.positive.value() as i64 - self.negative.value() as i64
    }

    /// Merge with another PN-Counter.
    pub fn merge(&mut self, other: &PNCounter) {
        self.positive.merge(&other.positive);
        self.negative.merge(&other.negative);
    }

    /// Create a merged copy.
    pub fn merged(&self, other: &PNCounter) -> PNCounter {
        let mut result = self.clone();
        result.merge(other);
        result
    }
}

impl Default for PNCounter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_value() {
        let c = PNCounter::new();
        assert_eq!(c.value(), 0);
    }

    #[test]
    fn test_increment() {
        let mut c = PNCounter::new();
        c.increment(1);
        c.increment(1);
        assert_eq!(c.value(), 2);
    }

    #[test]
    fn test_decrement() {
        let mut c = PNCounter::new();
        c.decrement(1);
        assert_eq!(c.value(), -1);
    }

    #[test]
    fn test_increment_decrement() {
        let mut c = PNCounter::new();
        c.increment(1);
        c.increment(1);
        c.decrement(1);
        assert_eq!(c.value(), 1);
    }

    #[test]
    fn test_merge() {
        let mut c1 = PNCounter::new();
        c1.increment(1);
        let mut c2 = PNCounter::new();
        c2.decrement(2);
        let merged = c1.merged(&c2);
        assert_eq!(merged.value(), 0); // +1 - 1
    }

    #[test]
    fn test_merge_commutative() {
        let mut c1 = PNCounter::new();
        c1.increment(1);
        c1.decrement(1);
        let mut c2 = PNCounter::new();
        c2.increment(2);
        let m1 = c1.merged(&c2);
        let m2 = c2.merged(&c1);
        assert_eq!(m1, m2);
    }

    #[test]
    fn test_merge_idempotent() {
        let mut c = PNCounter::new();
        c.increment(1);
        c.decrement(1);
        let merged = c.merged(&c);
        assert_eq!(merged, c);
    }

    #[test]
    fn test_concurrent_increments() {
        let mut c1 = PNCounter::new();
        c1.increment(1);
        let mut c2 = PNCounter::new();
        c2.increment(2);
        let merged = c1.merged(&c2);
        assert_eq!(merged.value(), 2);
    }
}
