use std::time::Duration;
use localcache::lib::basiccache::BasicCache;
use localcache::lib::cache::Cache;

#[test]
fn test_basic_cache_insert_and_get() {
    let mut cache: BasicCache<String, String> = BasicCache::new();

    // 测试插入和获取
    cache.insert("key1".to_string(), "value1".to_string());
    assert_eq!(cache.get(&"key1".to_string()), Some("value1".to_string()));

    // 测试不存在的键
    assert_eq!(cache.get(&"nonexistent".to_string()), None);
}

#[test]
fn test_basic_cache_remove() {
    let mut cache: BasicCache<String, String> = BasicCache::new();

    cache.insert("key1".to_string(), "value1".to_string());
    assert_eq!(cache.get(&"key1".to_string()), Some("value1".to_string()));

    // 测试删除存在的键
    let removed_value = cache.remove(&"key1".to_string());
    assert_eq!(removed_value, Some("value1".to_string()));
    assert_eq!(cache.get(&"key1".to_string()), None);

    // 测试删除不存在的键
    let removed_value = cache.remove(&"nonexistent".to_string());
    assert_eq!(removed_value, None);
}

#[test]
fn test_basic_cache_clear() {
    let mut cache: BasicCache<String, String> = BasicCache::new();

    cache.insert("key1".to_string(), "value1".to_string());
    cache.insert("key2".to_string(), "value2".to_string());
    assert_eq!(cache.len(), 2);

    cache.clear();
    assert_eq!(cache.len(), 0);
    assert!(cache.is_empty());
}

#[test]
fn test_basic_cache_with_ttl() {
    let mut cache: BasicCache<String, String> = BasicCache::new();

    // 插入带TTL的值
    cache.insert_with_ttl(
        "key1".to_string(),
        "value1".to_string(),
        Some(Duration::from_millis(100)),
    );

    // 应该能获取到值
    assert_eq!(cache.get(&"key1".to_string()), Some("value1".to_string()));

    // 等待过期
    std::thread::sleep(Duration::from_millis(150));

    // 应该获取不到值（已过期）
    assert_eq!(cache.get(&"key1".to_string()), None);
}
