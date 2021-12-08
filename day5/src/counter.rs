use std::collections::HashMap;
use std::hash::Hash;

/// Counts the frequencies with which items appear in a set.
pub struct Counter<K>(pub HashMap<K, u16>);

impl<K> Default for Counter<K> {
    fn default() -> Self {
        Self(HashMap::new())
    }
}

impl<K: Eq + PartialEq + Hash> Counter<K> {
    /// Add one item to the counter, recording its frequeuncy.
    pub fn add(&mut self, key: K) {
        *self.0.entry(key).or_insert(0) += 1;
    }
    /// How many items have count greater than or equal to the given number?
    pub fn count_ge(self, count: u16) -> usize {
        self.0.iter().filter(|(_k, v)| **v >= count).count()
    }
}
