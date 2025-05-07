use std::{hash::Hash, ops::Deref, sync::Arc};

use dashmap::DashMap;

#[derive(Clone)]
pub struct ConcurrentHashMap<K: Eq + Hash, V>(Arc<DashMap<K, V>>);

impl<K: Eq + Hash, V> Deref for ConcurrentHashMap<K, V> {
    type Target = Arc<DashMap<K, V>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<K: Eq + Hash, V> ConcurrentHashMap<K, V> {
    pub fn new() -> Self {
        Self(Arc::new(DashMap::new()))
    }
}
