use std::time::Duration;
use localcache::lib::ttlcache::TtlCache;
use localcache::lib::cache::Cache;
#[test]
fn test_ttl_cache_insert_and_get() {
    let ttl = Duration::from_millis(200);
    let mut cache: TtlCache<String, String> = TtlCache::new(ttl);
    
    // 测试插入和获取
    cache.insert("key1".to_string(), "value1".to_string());
    assert_eq!(cache.get(&"key1".to_string()), Some("value1".to_string()));
    
    // 测试不存在的键
    assert_eq!(cache.get(&"nonexistent".to_string()), None);
}

#[test]
fn test_ttl_cache_default_ttl() {
    let ttl = Duration::from_millis(100);
    let mut cache: TtlCache<String, i32> = TtlCache::new(ttl);
    
    // 插入不带特定TTL的值，应该使用默认TTL
    cache.insert("counter".to_string(), 42);
    assert_eq!(cache.get(&"counter".to_string()), Some(42));
    
    // 等待过期
    std::thread::sleep(Duration::from_millis(150));
    
    // 应该获取不到值（已过期）
    assert_eq!(cache.get(&"counter".to_string()), None);
}

#[test]
fn test_ttl_cache_custom_ttl() {
    let default_ttl = Duration::from_millis(500);
    let mut cache: TtlCache<String, String> = TtlCache::new(default_ttl);
    
    // 使用自定义TTL插入值（比默认TTL短）
    cache.insert_with_ttl(
        "key1".to_string(), 
        "value1".to_string(), 
        Some(Duration::from_millis(100))
    );
    
    // 应该能获取到值
    assert_eq!(cache.get(&"key1".to_string()), Some("value1".to_string()));
    
    // 等待自定义TTL过期
    std::thread::sleep(Duration::from_millis(150));
    
    // 应该获取不到值（已过期）
    assert_eq!(cache.get(&"key1".to_string()), None);
}

#[test]
fn test_ttl_cache_remove() {
    let ttl = Duration::from_secs(10);
    let mut cache: TtlCache<String, String> = TtlCache::new(ttl);
    
    cache.insert("key1".to_string(), "value1".to_string());
    assert_eq!(cache.get(&"key1".to_string()), Some("value1".to_string()));
    
    // 测试删除存在的键
    let removed_value = cache.remove(&"key1".to_string());
    assert_eq!(removed_value, Some("value1".to_string()));
    assert_eq!(cache.get(&"key1".to_string()), None);
}