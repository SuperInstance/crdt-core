//! Observed-Remove Set (OR-Set): supports both add and remove with causal semantics.

use std::collections::{HashMap, HashSet};

/// Unique tag for an add operation.
pub type Tag = u64;

/// An OR-Set CRDT. Each add gets a unique tag; remove only removes observed tags.
#[derive(Debug, Clone)]
pub struct ORSet<T: Clone + Eq + std::hash::Hash> {
    /// element -> set of unique add tags
    adds: HashMap<T, HashSet<Tag>>,
    /// Set of removed tags (tombstones)
    tombstones: HashSet<Tag>,
    next_tag: Tag,
}

impl<T: Clone + Eq + std::hash::Hash> ORSet<T> {
    pub fn new() -> Self {
        Self {
            adds: HashMap::new(),
            tombstones: HashSet::new(),
            next_tag: 0,
        }
    }

    /// Add an element. Returns the unique tag used.
    pub fn add(&mut self, element: T) -> Tag {
        let tag = self.next_tag;
        self.next_tag += 1;
        self.adds.entry(element).or_default().insert(tag);
        tag
    }

    /// Add with a specific tag (for merging).
    pub fn add_with_tag(&mut self, element: T, tag: Tag) {
        if !self.tombstones.contains(&tag) {
            self.adds.entry(element).or_default().insert(tag);
        }
        self.next_tag = std::cmp::max(self.next_tag, tag + 1);
    }

    /// Remove an element (only removes currently observed tags).
    pub fn remove(&mut self, element: &T) -> bool {
        if let Some(tags) = self.adds.remove(element) {
            self.tombstones.extend(tags);
            return true;
        }
        false
    }

    /// Check if element is present (has non-tombstoned tags).
    pub fn contains(&self, element: &T) -> bool {
        match self.adds.get(element) {
            Some(tags) => {
                let live: HashSet<_> = tags.difference(&self.tombstones).copied().collect();
                !live.is_empty()
            }
            None => false,
        }
    }

    /// Get all live elements.
    pub fn elements(&self) -> Vec<T> {
        self.adds.iter()
            .filter(|(_, tags)| {
                tags.iter().any(|t| !self.tombstones.contains(t))
            })
            .map(|(e, _)| e.clone())
            .collect()
    }

    /// Merge with another OR-Set.
    pub fn merge(&mut self, other: &ORSet<T>) {
        // Union tombstones
        self.tombstones.extend(other.tombstones.clone());

        // Union adds
        for (elem, tags) in &other.adds {
            let entry = self.adds.entry(elem.clone()).or_default();
            for tag in tags {
                if !self.tombstones.contains(tag) {
                    entry.insert(*tag);
                }
            }
        }

        // Clean up tombstoned entries
        let tombstones = &self.tombstones;
        self.adds.retain(|_, tags| {
            tags.iter().any(|t| !tombstones.contains(t))
        });

        self.next_tag = std::cmp::max(self.next_tag, other.next_tag);
    }

    /// Number of live elements.
    pub fn len(&self) -> usize {
        self.elements().len()
    }

    /// Is the set empty?
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<T: Clone + Eq + std::hash::Hash> Default for ORSet<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_and_contains() {
        let mut s = ORSet::new();
        s.add(42);
        assert!(s.contains(&42));
    }

    #[test]
    fn test_remove() {
        let mut s = ORSet::new();
        s.add(1);
        s.remove(&1);
        assert!(!s.contains(&1));
    }

    #[test]
    fn test_remove_nonexistent() {
        let mut s = ORSet::new();
        assert!(!s.remove(&99));
    }

    #[test]
    fn test_add_back_after_remove() {
        let mut s = ORSet::new();
        s.add(1);
        s.remove(&1);
        s.add(1);
        assert!(s.contains(&1));
    }

    #[test]
    fn test_concurrent_add_remove() {
        // Replica 1: add(A) with tag 0, remove(A)
        let mut r1 = ORSet::new();
        r1.add("A"); // tag 0
        r1.remove(&"A"); // tombstone tag 0

        // Replica 2: add(A) with tag 100 (simulating different origin)
        let mut r2 = ORSet::new();
        r2.add_with_tag("A", 100);

        // Merge: r2's add wins because r1 didn't see r2's tag
        r1.merge(&r2);
        assert!(r1.contains(&"A"));
    }

    #[test]
    fn test_merge_union() {
        let mut s1 = ORSet::new();
        s1.add(1);
        let mut s2 = ORSet::new();
        s2.add(2);
        s1.merge(&s2);
        assert!(s1.contains(&1));
        assert!(s1.contains(&2));
    }

    #[test]
    fn test_elements_list() {
        let mut s = ORSet::new();
        s.add(1);
        s.add(2);
        s.add(3);
        let elems = s.elements();
        assert_eq!(elems.len(), 3);
    }

    #[test]
    fn test_merge_with_tombstones() {
        let mut s1 = ORSet::new();
        let tag = s1.add(1);
        let mut s2 = ORSet::new();
        s2.add_with_tag(1, tag);
        s2.remove(&1);
        s1.merge(&s2);
        assert!(!s1.contains(&1));
    }
}
