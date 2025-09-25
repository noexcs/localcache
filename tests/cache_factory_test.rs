use localcache::lib::cache::{CacheType, new_cache};
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
fn test_new_cache_lru() {
    let mut cache = new_cache::<String, i32>(CacheType::Lru(100));

    cache.insert("lru_key".to_string(), 100);
    assert_eq!(cache.get(&"lru_key".to_string()), Some(100));

    let removed_value = cache.remove(&"lru_key".to_string());
    assert_eq!(removed_value, Some(100));
    assert_eq!(cache.get(&"lru_key".to_string()), None);
}
