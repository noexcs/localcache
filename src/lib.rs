// src/lib.rs
use std::collections::HashMap;
use std::time::{SystemTime, Duration};
use std::hash::Hash;

/// 缓存类型枚举，用于指定不同的缓存实现
#[derive(Debug, Clone)]
pub enum CacheType {
    /// 基础的 HashMap 实现
    Basic,
    /// LRU (Least Recently Used) 实现
    Lru(usize), // usize 表示最大容量
    /// 带默认过期时间的实现
    WithDefaultTtl(Duration),
}

/// 缓存 trait，定义缓存的基本操作
pub trait Cache<K, V> 
where 
    K: Hash + Eq,
    V: Clone,
{
    fn insert(&mut self, key: K, value: V);
    fn insert_with_ttl(&mut self, key: K, value: V, ttl: Option<Duration>);
    fn get(&self, key: &K) -> Option<V>;
    fn remove(&mut self, key: &K) -> Option<V>;
    fn clear(&mut self);
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
}

/// 基础缓存实现
pub struct BasicCache<K, V> {
    data: HashMap<K, CacheEntry<V>>,
    default_ttl: Option<Duration>,
}

/// LRU缓存实现
pub struct LruCache<K, V> {
    data: HashMap<K, CacheEntry<V>>,
    default_ttl: Option<Duration>,
    #[warn(unused_variables)]
    _max_size: usize,
}

/// 带默认TTL的缓存实现
pub struct TtlCache<K, V> {
    data: HashMap<K, CacheEntry<V>>,
    default_ttl: Duration,
}

/// 缓存条目，包含值和过期时间
#[derive(Clone)]
struct CacheEntry<T> {
    value: T,
    expiry: Option<SystemTime>,
}

impl<K, V> BasicCache<K, V>
where
    K: Hash + Eq,
    V: Clone,
{
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
            default_ttl: None,
        }
    }
}

impl<K, V> Cache<K, V> for BasicCache<K, V>
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

/// 创建指定类型的缓存实现
pub fn new_cache<K, V>(cache_type: CacheType) -> Box<dyn Cache<K, V>>
where
    K: Hash + Eq + Clone + 'static,
    V: Clone + 'static,
{
    match cache_type {
        CacheType::Basic => Box::new(BasicCache::new()),
        CacheType::Lru(max_size) => Box::new(LruCache::new(max_size)),
        CacheType::WithDefaultTtl(duration) => Box::new(TtlCache::new(duration)),
    }
}