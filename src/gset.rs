//! Grow-only set (G-Set): add-only set where merge is union.

use std::collections::HashSet;

/// A G-Set CRDT. Elements can only be added, never removed.
/// Merge is set union.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GSet<T: Clone + Eq + std::hash::Hash> {
    elements: HashSet<T>,
}

impl<T: Clone + Eq + std::hash::Hash> GSet<T> {
    pub fn new() -> Self {
        Self { elements: HashSet::new() }
    }

    /// Add an element.
    pub fn add(&mut self, element: T) -> bool {
        self.elements.insert(element)
    }

    /// Check if element is present.
    pub fn contains(&self, element: &T) -> bool {
        self.elements.contains(element)
    }

    /// Get all elements.
    pub fn elements(&self) -> &HashSet<T> {
        &self.elements
    }

    /// Number of elements.
    pub fn len(&self) -> usize {
        self.elements.len()
    }

    /// Is the set empty?
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    /// Merge with another G-Set (union).
    pub fn merge(&mut self, other: &GSet<T>) {
        for elem in &other.elements {
            self.elements.insert(elem.clone());
        }
    }

    /// Create a merged copy.
    pub fn merged(&self, other: &GSet<T>) -> GSet<T> {
        let mut result = self.clone();
        result.merge(other);
        result
    }
}

impl<T: Clone + Eq + std::hash::Hash> Default for GSet<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_and_contains() {
        let mut s = GSet::new();
        s.add(42);
        assert!(s.contains(&42));
        assert!(!s.contains(&99));
    }

    #[test]
    fn test_add_idempotent() {
        let mut s = GSet::new();
        assert!(s.add(1));
        assert!(!s.add(1));
        assert_eq!(s.len(), 1);
    }

    #[test]
    fn test_merge_union() {
        let mut s1 = GSet::new();
        s1.add(1);
        s1.add(2);
        let mut s2 = GSet::new();
        s2.add(2);
        s2.add(3);
        let merged = s1.merged(&s2);
        assert_eq!(merged.len(), 3);
        assert!(merged.contains(&1));
        assert!(merged.contains(&2));
        assert!(merged.contains(&3));
    }

    #[test]
    fn test_merge_idempotent() {
        let mut s = GSet::new();
        s.add(1);
        let merged = s.merged(&s);
        assert_eq!(merged, s);
    }

    #[test]
    fn test_merge_commutative() {
        let mut s1 = GSet::new();
        s1.add(1);
        let mut s2 = GSet::new();
        s2.add(2);
        let m1 = s1.merged(&s2);
        let m2 = s2.merged(&s1);
        assert_eq!(m1, m2);
    }

    #[test]
    fn test_string_elements() {
        let mut s = GSet::new();
        s.add("hello".to_string());
        s.add("world".to_string());
        assert_eq!(s.len(), 2);
    }
}
