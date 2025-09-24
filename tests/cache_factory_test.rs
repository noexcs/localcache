use localcache::{CacheType, new_cache, Cache};
use std::time::Duration;

#[test]
fn test_new_cache_basic() {
    let mut cache = new_cache::<String, String>(CacheType::Basic);
    
    cache.insert("key1".to_string(), "value1".to_string());
    assert_eq!(cache.get(&"key1".to_string()), Some("value1".to_string()));
    
    let removed_value = cache.remove(&"key1".to_string());
    assert_eq!(removed_value, Some("value1".to_string()));
    assert_eq!(cache.get(&"key1".to_string()), None);
}

#[test]
fn test_new_cache_with_default_ttl() {
    let mut cache = new_cache::<String, i32>(
        CacheType::WithDefaultTtl(Duration::from_millis(100))
    );
    
    cache.insert("counter".to_string(), 42);
    assert_eq!(cache.get(&"counter".to_string()), Some(42));
    
    // 等待过期
    std::thread::sleep(Duration::from_millis(150));
    
    // 应该获取不到值（已过期）
    assert_eq!(cache.get(&"counter".to_string()), None);
}

#[test]
fn test_new_cache_lru() {
    let mut cache = new_cache::<String, i32>(CacheType::Lru(100));
    
    cache.insert("lru_key".to_string(), 100);
    assert_eq!(cache.get(&"lru_key".to_string()), Some(100));
    
    let removed_value = cache.remove(&"lru_key".to_string());
    assert_eq!(removed_value, Some(100));
    assert_eq!(cache.get(&"lru_key".to_string()), None);
}