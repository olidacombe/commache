//! A general command output cache.
//!
//! This is a work in progress, and in its current form
//! leaves orphan/zombie processes behing on every invokation.
//!
//! ## TODO
//!
//! Transition to a daemon implementation, where the client
//! queries the daemon, and the daemon is responsible for the
//! lazy after-return fetching.

use lazy_static::lazy_static;
use runner::{Runner, RunnerImpl};

use crate::cache::rocks::RocksDbCache;
use std::{
    fmt::Display,
    io::{stdout, Write},
};

pub mod cache;
pub mod cli;
pub mod config;
pub mod error;
pub mod key;
mod runner;
pub mod server;

lazy_static! {
    pub static ref CONFIG: config::CommacheConfig = config::get();
}

fn print_now<T: Display>(printme: T) {
    print!("{}", printme);
    stdout().flush().unwrap();
}

pub fn main(args: cli::Args) {
    let cache = RocksDbCache::new(&CONFIG.db_dir);
    let runner = RunnerImpl::new(args, cache);
    runner.get(print_now);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
