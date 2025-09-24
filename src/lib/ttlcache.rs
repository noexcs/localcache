
use std::collections::HashMap;
use std::hash::Hash;
use std::time::{Duration, SystemTime};


use crate::lib::cache::{Cache, CacheEntry};

pub struct TtlCache<K, V> {
    data: HashMap<K, CacheEntry<V>>,
    default_ttl: Duration,
}

impl<K, V> TtlCache<K, V>
where
    K: Hash + Eq,
    V: Clone,
{
    pub fn new(default_ttl: Duration) -> Self {
        Self {
            data: HashMap::new(),
            default_ttl,
        }
    }
}

impl<K, V> Cache<K, V> for TtlCache<K, V>
where
    K: Hash + Eq,
    V: Clone,
{
    fn insert(&mut self, key: K, value: V) {
        self.insert_with_ttl(key, value, Some(self.default_ttl));
    }

    fn insert_with_ttl(&mut self, key: K, value: V, ttl: Option<Duration>) {
        let expiry = ttl.map(|duration| SystemTime::now() + duration);
        self.data.insert(key, CacheEntry { value, expiry });
    }

    fn get(&self, key: &K) -> Option<V> {
        match self.data.get(key) {
            Some(entry) => {
                // 检查是否过期
                if let Some(expiry) = entry.expiry {
                    if SystemTime::now() > expiry {
                        return None; // 已过期
                    }
                }
                Some(entry.value.clone())
            }
            None => None,
        }
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        self.data.remove(key).map(|entry| entry.value)
    }

    fn clear(&mut self) {
        self.data.clear();
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}
