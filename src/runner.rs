use std::process::Command;

use crate::{cache::Cache, cli::Args, key::ToKey, server};

pub trait Runner {
    type Cache: Cache<Vec<u8>>;

    fn mv(self) -> (Args, Self::Cache);
    fn new(args: Args, cache: Self::Cache) -> Self;
    fn get<F: Fn(String)>(self, cb: F);
    fn run_and_cache(self) -> (Vec<u8>, Self::Cache);
}

pub struct RunnerImpl<K: Send + 'static, C: Cache<K>> {
    args: Args,
    cache: C,
    key: K,
}

fn run(cmd: &str, args: &[&str]) -> Option<Vec<u8>> {
    Command::new(cmd).args(args).output().map(|o| o.stdout).ok()
}

impl<C> Runner for RunnerImpl<Vec<u8>, C>
where
    C: Cache<Vec<u8>>,
{
    type Cache = C;

    fn mv(self) -> (Args, Self::Cache) {
        (self.args, self.cache)
    }

    fn new(args: Args, cache: C) -> Self {
        let key = args.get().key();
        Self { args, cache, key }
    }

    fn get<F: Fn(String)>(self, cb: F) {
        let v = self.cache.get(&self.key);
        if let Some(v) = v {
            cb(v);
            // refresh cache in the background
            server::queue(self);
        } else {
            let (v, _) = self.run_and_cache();
            let v = String::from_utf8(v).ok().unwrap_or_else(String::new);
            cb(v);
        }
    }

    fn run_and_cache(mut self) -> (Vec<u8>, C) {
        let args = self.args.get();
        if let Some((cmd, args)) = args.split_first() {
            if let Some(out) = run(cmd, args) {
                self.cache.patch(&self.key, &out);
                return (out, self.cache);
            }
        }
        (Vec::new(), self.cache)
    }
}
