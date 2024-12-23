use std::{ops::Deref, sync::Arc};

use bytes::Bytes;
use dashmap::DashMap;

use crate::pb::kv::KeyVal;

#[derive(Debug, Clone)]
pub struct KvStore {
    // arc mutex is to allow multiple client to access the inner data store
    inner: Arc<KvStoreInner>,
}

impl Deref for KvStore {
    type Target = KvStoreInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[derive(Debug, Clone)]
pub struct KvStoreInner {
    pub dashmap: DashMap<String, Bytes>,
}

impl KvStore {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(KvStoreInner {
                dashmap: DashMap::with_capacity(100),
            }),
        }
    }

    pub fn get(&self, key: String) -> Option<Bytes> {
        self.dashmap.get(&key).map(|v| v.value().clone())
    }

    pub fn set(&mut self, kv: KeyVal) {
        self.dashmap.insert(kv.key, kv.value);
    }

    pub fn del(&mut self, key: String) {
        self.dashmap.remove(&key);
    }
}
