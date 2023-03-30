//! Commache!

use key::ToKey;
use lazy_static::lazy_static;
use rocksdb::SingleThreaded;

use crate::cache::{rocks::RocksDbCache, Cache};

pub mod cache;
pub mod config;
pub mod key;

lazy_static! {
    static ref CONFIG: config::CommacheConfig = config::get();
    static ref CACHE: RocksDbCache<SingleThreaded> = RocksDbCache::new(&CONFIG.db_dir);
}

pub fn run(args: &[&str]) {
    let key = args.key();
    let v = CACHE.get(&key);
    dbg!(v);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
