//! Grow-only counter (G-Counter): each replica increments its own slot, merge takes max.

use std::collections::HashMap;

/// A G-Counter CRDT. Each node increments only its own entry.
/// Merge takes the component-wise maximum.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GCounter {
    counts: HashMap<u64, u64>,
}

impl GCounter {
    pub fn new() -> Self {
        Self { counts: HashMap::new() }
    }

    /// Increment the counter for a given node.
    pub fn increment(&mut self, node_id: u64) -> u64 {
        let entry = self.counts.entry(node_id).or_insert(0);
        *entry += 1;
        *entry
    }

    /// Get the total count across all nodes.
    pub fn value(&self) -> u64 {
        self.counts.values().sum()
    }

    /// Merge with another G-Counter (component-wise max).
    pub fn merge(&mut self, other: &GCounter) {
        for (&node, &count) in &other.counts {
            let entry = self.counts.entry(node).or_insert(0);
            *entry = std::cmp::max(*entry, count);
        }
    }

    /// Create a merged copy.
    pub fn merged(&self, other: &GCounter) -> GCounter {
        let mut result = self.clone();
        result.merge(other);
        result
    }

    /// Get the count for a specific node.
    pub fn get(&self, node_id: u64) -> u64 {
        *self.counts.get(&node_id).unwrap_or(&0)
    }
}

impl Default for GCounter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_value() {
        let c = GCounter::new();
        assert_eq!(c.value(), 0);
    }

    #[test]
    fn test_increment() {
        let mut c = GCounter::new();
        c.increment(1);
        c.increment(1);
        assert_eq!(c.value(), 2);
        assert_eq!(c.get(1), 2);
    }

    #[test]
    fn test_increment_different_nodes() {
        let mut c = GCounter::new();
        c.increment(1);
        c.increment(2);
        c.increment(3);
        assert_eq!(c.value(), 3);
    }

    #[test]
    fn test_merge() {
        let mut c1 = GCounter::new();
        c1.increment(1);
        c1.increment(1);
        let mut c2 = GCounter::new();
        c2.increment(2);
        c2.increment(2);
        c2.increment(2);
        let merged = c1.merged(&c2);
        assert_eq!(merged.value(), 5);
    }

    #[test]
    fn test_merge_idempotent() {
        let mut c = GCounter::new();
        c.increment(1);
        c.increment(1);
        let merged = c.merged(&c);
        assert_eq!(merged.value(), 2);
    }

    #[test]
    fn test_merge_commutative() {
        let mut c1 = GCounter::new();
        c1.increment(1);
        let mut c2 = GCounter::new();
        c2.increment(2);
        let m1 = c1.merged(&c2);
        let m2 = c2.merged(&c1);
        assert_eq!(m1, m2);
    }

    #[test]
    fn test_merge_associative() {
        let mut c1 = GCounter::new(); c1.increment(1);
        let mut c2 = GCounter::new(); c2.increment(2);
        let mut c3 = GCounter::new(); c3.increment(3);
        let m12 = c1.merged(&c2);
        let m12_3 = m12.merged(&c3);
        let m23 = c2.merged(&c3);
        let m1_23 = c1.merged(&m23);
        assert_eq!(m12_3, m1_23);
    }
}
