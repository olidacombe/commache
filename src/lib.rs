//! Commache!

use key::ToKey;
use lazy_static::lazy_static;

use crate::cache::{rocks::RocksDbCache, Cache};

pub mod cache;
pub mod cli;
pub mod config;
pub mod key;
mod runner;

lazy_static! {
    static ref CONFIG: config::CommacheConfig = config::get();
}

pub fn main(args: cli::Args) {
    let cache = RocksDbCache::new(&CONFIG.db_dir);
    let key = args.get().key();
    let v = cache.get(&key);
    if let Some(v) = v {
        println!("{}", v);
    }

    runner::spawn(args, cache, key);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
