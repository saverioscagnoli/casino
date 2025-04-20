use async_lock::RwLock;
use std::{collections::HashMap, hash::Hash, ops::Deref, sync::Arc};

#[derive(Clone)]
pub struct Cache<K: Hash + Eq, V>(Arc<RwLock<HashMap<K, V>>>);

impl<K: Hash + Eq, V> Deref for Cache<K, V> {
    type Target = Arc<RwLock<HashMap<K, V>>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<K: Hash + Eq, V> Cache<K, V> {
    pub fn new() -> Self {
        Self(Arc::new(RwLock::new(HashMap::new())))
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self(Arc::new(RwLock::new(HashMap::with_capacity(capacity))))
    }

    pub async fn insert(&self, key: K, value: V) -> Option<V> {
        let mut lock = self.0.write().await;
        lock.insert(key, value)
    }

    pub async fn remove(&self, key: &K) -> Option<V> {
        let mut lock = self.0.write().await;
        lock.remove(key)
    }

    pub async fn len(&self) -> usize {
        let lock = self.0.read().await;
        lock.len()
    }
}
