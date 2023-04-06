use std::process::Command;

use crate::{cache::Cache, cli::Args, key::ToKey, server};

pub struct Runner<K: Send + 'static, C: Cache<K>> {
    args: Args,
    cache: C,
    key: K,
}

fn run(cmd: &str, args: &[&str]) -> Option<Vec<u8>> {
    Command::new(cmd).args(args).output().map(|o| o.stdout).ok()
}

impl<C> Runner<Vec<u8>, C>
where
    C: Cache<Vec<u8>>,
{
    pub fn new(args: Args, cache: C) -> Self {
        let key = args.get().key();
        Self { args, cache, key }
    }

    pub fn get(self) -> String {
        let v = self.cache.get(&self.key);
        if let Some(v) = v {
            // // print out what we already have cached
            // print_now(v);

            // refresh cache in the background
            server::queue(self.args, self.cache, self.key);
            return v;
        } else {
            let v = self.run_and_cache();
            return String::from_utf8(v).ok().unwrap_or_else(String::new);
        }
    }

    pub fn run_and_cache(mut self) -> Vec<u8> {
        let args = self.args.get();
        if let Some((cmd, args)) = args.split_first() {
            if let Some(out) = run(cmd, args) {
                self.cache.patch(&self.key, &out);
                return out;
            }
        }
        Vec::new()
    }
}
