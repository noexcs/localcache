
use std::collections::HashMap;
use std::time::{SystemTime, Duration};
use std::hash::Hash;

use crate::lib::cache::{Cache, CacheEntry};


pub struct LruCache<K, V> {
    data: HashMap<K, CacheEntry<V>>,
    default_ttl: Option<Duration>,
    _max_size: usize,
}

impl<K, V> LruCache<K, V>
where
    K: Hash + Eq,
    V: Clone,
{
    pub fn new(max_size: usize) -> Self {
        Self {
            data: HashMap::new(),
            default_ttl: None,
            _max_size: max_size,
        }
    }
}

impl<K, V> Cache<K, V> for LruCache<K, V>
where
    K: Hash + Eq,
    V: Clone,
{
    fn insert(&mut self, key: K, value: V) {
        self.insert_with_ttl(key, value, self.default_ttl);
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