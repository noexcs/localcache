use std::time::Duration;

use localcache::lib::cache::{new_cache, CacheType};

fn main() {
    // 创建一个简单的缓存
    let mut cache = new_cache::<String, String>(CacheType::Basic);
    // 插入数据
    cache.insert("key1".to_string(), "value1".to_string());

    // 获取数据
    if let Some(value) = cache.get(&"key1".to_string()) {
        println!("获取到值: {}", value);
    }

    // 创建带默认过期时间的缓存
    let mut cache_with_ttl = new_cache::<String, i32>(
        CacheType::WithDefaultTtl(Duration::from_secs(10))
    );

    cache_with_ttl.insert("counter".to_string(), 42);
    println!("Counter value: {:?}", cache_with_ttl.get(&"counter".to_string()));

    // 创建LRU缓存
    let mut lru_cache = new_cache::<String, i32>(CacheType::Lru(100));
    lru_cache.insert("lru_key".to_string(), 100);
    println!("LRU cache value: {:?}", lru_cache.get(&"lru_key".to_string()));

    println!("Hello, world!");
}