use std::time::Duration;

pub struct Cache<K, V> {
    max_entries: usize,
    _marker: std::marker::PhantomData<(K, V)>,
}

pub struct CacheConfig {
    pub max_entries: usize,
}

impl<K, V> Cache<K, V> {
    pub fn new(config: CacheConfig) -> Self {
        Self { max_entries: config.max_entries, _marker: std::marker::PhantomData }
    }
    
    pub fn set(&self, _key: K, _value: V, _ttl: Duration) {}
    pub fn get(&self, _key: &K) -> Option<&V> { None }
    pub fn len(&self) -> usize { 0 }
}
