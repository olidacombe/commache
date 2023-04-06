use fork::{daemon, Fork};
use std::process::Command;

use crate::{cache::Cache, cli};

fn run(cmd: &str, args: &[&str]) -> Option<Vec<u8>> {
    Command::new(cmd).args(args).output().map(|o| o.stdout).ok()
}

pub fn run_and_cache<K: Send + 'static>(
    args: cli::Args,
    mut cache: impl Cache<K>,
    key: K,
) -> Vec<u8> {
    let args = args.get();
    if let Some((cmd, args)) = args.split_first() {
        if let Some(out) = run(cmd, args) {
            cache.patch(key, &out);
            return out;
        }
    }
    Vec::new()
}

// pub fn spawn<K: Send + 'static>(args: cli::Args, mut cache: impl Cache<K>, key: K) {
//     if let Ok(Fork::Child) = daemon(true, false) {
//         run_and_cache(args, cache, key);
//     }
// }
