//! Commache!

use key::ToKey;
use lazy_static::lazy_static;
use rocksdb::SingleThreaded;

use crate::cache::{rocks::RocksDbCache, Cache};

pub mod cache;
pub mod cli;
pub mod config;
pub mod key;
mod runner;

lazy_static! {
    static ref CONFIG: config::CommacheConfig = config::get();
    static ref CACHE: RocksDbCache<SingleThreaded> = RocksDbCache::new(&CONFIG.db_dir);
}

pub fn main(args: &[&str]) {
    let key = args.key();
    let v = CACHE.get(&key);
    dbg!(v);

    if let Some((cmd, args)) = args.split_first() {
        runner::run(cmd, args);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
