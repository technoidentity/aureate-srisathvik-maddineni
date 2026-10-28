use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub struct HashTable<K, V> {
    buckets: Vec<Vec<(K, V)>>,
}

impl<K: Hash + Eq, V> HashTable<K, V> {
    pub fn new() -> Self {
        Self {
            buckets: (0..16).map(|_| Vec::new()).collect(),
        }
    }

    fn bucket_index(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() as usize) % self.buckets.len()
    }

    pub fn insert(&mut self, key: K, value: V) {
        let index = self.bucket_index(&key);

        for entry in &mut self.buckets[index] {
            if entry.0 == key {
                entry.1 = value;
                return;
            }
        }

        self.buckets[index].push((key, value));
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        let index = self.bucket_index(key);

        self.buckets[index]
            .iter()
            .find(|entry| &entry.0 == key)
            .map(|entry| &entry.1)
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        let index = self.bucket_index(key);
        let bucket = &mut self.buckets[index];

        let position = bucket.iter().position(|entry| &entry.0 == key)?;

        Some(bucket.swap_remove(position).1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inserts_and_retrieves_values() {
        let mut table = HashTable::new();
        table.insert("Alice", 90);
        assert_eq!(table.get(&"Alice"), Some(&90));
    }

    #[test]
    fn updates_existing_key() {
        let mut table = HashTable::new();
        table.insert("Bob", 70);
        table.insert("Bob", 85);
        assert_eq!(table.get(&"Bob"), Some(&85));
    }

    #[test]
    fn returns_none_for_missing_key() {
        let table: HashTable<&str, i32> = HashTable::new();
        assert_eq!(table.get(&"Unknown"), None);
    }

    #[test]
    fn removes_key() {
        let mut table = HashTable::new();
        table.insert("Alice", 90);
        assert_eq!(table.remove(&"Alice"), Some(90));
        assert_eq!(table.get(&"Alice"), None);
    }
}
