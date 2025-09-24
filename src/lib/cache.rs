use crate::lib::basiccache::BasicCache;
use crate::lib::lrucache::LruCache;
use crate::lib::ttlcache::TtlCache;
use std::hash::Hash;
use std::time::{Duration, SystemTime};

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

/// 缓存条目，包含值和过期时间
#[derive(Clone)]
pub(crate) struct CacheEntry<T> {
    pub(crate) value: T,
    pub(crate) expiry: Option<SystemTime>,
}
