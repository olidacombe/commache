use super::Cache;
use rocksdb::{DBWithThreadMode, SingleThreaded, ThreadMode, DB};
use std::path::Path;
use tracing::debug;

pub struct RocksDbCache<T>
where
    T: ThreadMode,
{
    db: DBWithThreadMode<T>,
}

impl RocksDbCache<SingleThreaded> {
    pub fn new<P: AsRef<Path>>(db_dir: P) -> Self {
        let db = DB::open_default(db_dir).unwrap();
        Self { db }
    }
}

impl<K, T> Cache<K> for RocksDbCache<T>
where
    K: AsRef<[u8]> + Send,
    T: ThreadMode,
{
    fn get(&self, key: K) -> Option<String> {
        self.db
            .get(key)
            .unwrap_or(None)
            .map_or(None, |bytes| String::from_utf8(bytes).ok())
    }
    fn patch(&mut self, key: K, value: &[u8]) {
        if let Err(e) = self.db.put(key, value) {
            debug!("{:?}", e);
        }
    }
}
