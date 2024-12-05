use anyhow::Result;
use uuid::Uuid;

/// In async processing, precise order is not guaranteed.
///
/// 非同期処理では、正確な順序は保証されない
pub struct DynamicallySizedLRUCache<K: Eq + std::hash::Hash + Clone, V: Clone> {
    last_accessed_order: std::collections::BTreeMap<Uuid, K>,
    last_accessed_time: std::collections::HashMap<K, Uuid>,
    cache: std::collections::HashMap<K, V>,
}

impl<K: Eq + std::hash::Hash + Clone, V: Clone> DynamicallySizedLRUCache<K, V> {
    pub fn new() -> Self {
        Self {
            last_accessed_order: std::collections::BTreeMap::new(),
            last_accessed_time: std::collections::HashMap::new(),
            cache: std::collections::HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        let unix_timestamp = self.get_unix_timestamp();
        self.last_accessed_order.insert(unix_timestamp, key.clone());
        self.last_accessed_time.insert(key.clone(), unix_timestamp);
        self.cache.insert(key, value);
    }

    pub fn get(&mut self, key: &K) -> Option<V> {
        let unix_timestamp = self.get_unix_timestamp();
        match self.cache.get(key) {
            Some(value) => {
                self.last_accessed_order
                    .remove(&self.last_accessed_time[key]);
                self.last_accessed_order.insert(unix_timestamp, key.clone());
                self.last_accessed_time.insert(key.clone(), unix_timestamp);
                Some(value.clone())
            }
            None => None,
        }
    }

    pub fn remove_one(&mut self) -> Result<()> {
        let (oldest_timestamp, oldest_key) = self
            .last_accessed_order
            .iter()
            .nth(0 as usize)
            .ok_or(anyhow::anyhow!("Dynamic LRU: Cache is empty"))
            .map(|(timestamp, key)| (*timestamp, key.clone()))?;
        self.last_accessed_order.remove(&oldest_timestamp);
        self.last_accessed_time.remove(&oldest_key);
        self.cache.remove(&oldest_key);
        Ok(())
    }

    fn get_unix_timestamp(&self) -> Uuid {
        Uuid::now_v7()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dynamic_lru() {
        let mut cache = DynamicallySizedLRUCache::new();
        cache.insert(1, 1);
        cache.insert(2, 2);
        cache.insert(3, 3);
        assert_eq!(cache.get(&1), Some(1));
        assert_eq!(cache.get(&2), Some(2));
        assert_eq!(cache.get(&3), Some(3));
        cache.remove_one().unwrap();
        assert_eq!(cache.get(&1), None);
        assert_eq!(cache.get(&2), Some(2));
        assert_eq!(cache.get(&3), Some(3));
        cache.remove_one().unwrap();
        assert_eq!(cache.get(&1), None);
        assert_eq!(cache.get(&2), None);
        assert_eq!(cache.get(&3), Some(3));
        cache.remove_one().unwrap();
        assert_eq!(cache.get(&1), None);
        assert_eq!(cache.get(&2), None);
        assert_eq!(cache.get(&3), None);
    }
}
