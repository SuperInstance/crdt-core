//! Last-Writer-Wins (LWW) Register: resolves concurrent writes by timestamp.

/// A LWW Register CRDT. The value with the latest timestamp wins.
#[derive(Debug, Clone)]
pub struct LWWRegister<T: Clone> {
    value: T,
    timestamp: u64,
    node_id: u64, // tiebreaker
}

impl<T: Clone> LWWRegister<T> {
    pub fn new(value: T, timestamp: u64, node_id: u64) -> Self {
        Self { value, timestamp, node_id }
    }

    /// Get the current value.
    pub fn value(&self) -> &T {
        &self.value
    }

    /// Get the timestamp.
    pub fn timestamp(&self) -> u64 {
        self.timestamp
    }

    /// Write a new value with a timestamp.
    pub fn write(&mut self, value: T, timestamp: u64) {
        if timestamp >= self.timestamp {
            self.value = value;
            self.timestamp = timestamp;
        }
    }

    /// Write with tiebreaker (node_id breaks ties).
    pub fn write_with_node(&mut self, value: T, timestamp: u64, node_id: u64) {
        let should_update = timestamp > self.timestamp
            || (timestamp == self.timestamp && node_id > self.node_id);
        if should_update {
            self.value = value;
            self.timestamp = timestamp;
            self.node_id = node_id;
        }
    }

    /// Merge with another LWW register (higher timestamp wins).
    pub fn merge(&mut self, other: &LWWRegister<T>) {
        if other.timestamp > self.timestamp {
            self.value = other.value.clone();
            self.timestamp = other.timestamp;
            self.node_id = other.node_id;
        } else if other.timestamp == self.timestamp && other.node_id > self.node_id {
            self.value = other.value.clone();
            self.node_id = other.node_id;
        }
    }

    /// Create a merged copy.
    pub fn merged(&self, other: &LWWRegister<T>) -> LWWRegister<T> {
        let mut result = self.clone();
        result.merge(other);
        result
    }
}

impl<T: Clone + PartialEq> PartialEq for LWWRegister<T> {
    fn eq(&self, other: &Self) -> bool {
        self.timestamp == other.timestamp && self.value == other.value && self.node_id == other.node_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_register() {
        let r = LWWRegister::new(42, 100, 1);
        assert_eq!(*r.value(), 42);
        assert_eq!(r.timestamp(), 100);
    }

    #[test]
    fn test_write_newer() {
        let mut r = LWWRegister::new(1, 100, 1);
        r.write(2, 200);
        assert_eq!(*r.value(), 2);
    }

    #[test]
    fn test_write_older_ignored() {
        let mut r = LWWRegister::new(2, 200, 1);
        r.write(1, 100);
        assert_eq!(*r.value(), 2);
    }

    #[test]
    fn test_write_same_timestamp() {
        let mut r = LWWRegister::new(1, 100, 1);
        r.write(2, 100);
        assert_eq!(*r.value(), 2); // >= allows overwrite
    }

    #[test]
    fn test_merge_higher_wins() {
        let mut r1 = LWWRegister::new("a", 100, 1);
        let r2 = LWWRegister::new("b", 200, 2);
        r1.merge(&r2);
        assert_eq!(*r1.value(), "b");
    }

    #[test]
    fn test_merge_tiebreaker() {
        let mut r1 = LWWRegister::new("a", 100, 1);
        let r2 = LWWRegister::new("b", 100, 2);
        r1.merge(&r2);
        assert_eq!(*r1.value(), "b"); // node_id 2 > 1
    }

    #[test]
    fn test_merge_commutative() {
        let r1 = LWWRegister::new("a", 100, 1);
        let r2 = LWWRegister::new("b", 200, 2);
        let m1 = r1.merged(&r2);
        let m2 = r2.merged(&r1);
        assert_eq!(m1, m2);
    }

    #[test]
    fn test_write_with_node_tiebreaker() {
        let mut r = LWWRegister::new(1, 100, 1);
        r.write_with_node(2, 100, 5); // same ts, higher node
        assert_eq!(*r.value(), 2);
    }
}
