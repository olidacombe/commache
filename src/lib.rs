//! A general command output cache.
//!
//! This is a work in progress, and in its current form
//! leaves orphan/zombie processes behing on every invokation.
//!
//! ## TODO
//!
//! Transition to a daemon implementation, where the clien
//! queries the daemon, and the daemon is responsible for the
//! lazy after-return fetching.

use key::ToKey;
use lazy_static::lazy_static;

use crate::cache::{rocks::RocksDbCache, Cache};
use std::io::{stdout, Write};

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
        print!("{}", v);
    }

    stdout().flush().unwrap();

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
