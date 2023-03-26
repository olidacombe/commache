use super::Cache;
use rocksdb::{DBWithThreadMode, SingleThreaded, ThreadMode, DB};
use std::path::Path;
use tracing::Value;

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

impl<T> Cache for RocksDbCache<T>
where
    T: ThreadMode,
{
    fn get(&self, key: &str) -> Option<String> {
        self.db
            .get(key.as_bytes())
            .unwrap_or(None)
            .map_or(None, |bytes| String::from_utf8(bytes).ok())
    }
    fn patch(&mut self, key: &str, value: &str) {
        self.db.put(key.as_bytes(), value.as_bytes());
    }
}
