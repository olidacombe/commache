use fork::{fork, Fork};
use std::process::Command;
use tracing::{debug, error};

use crate::{cache::Cache, cli};

fn run(cmd: &str, args: &[&str]) -> Vec<u8> {
    let output = Command::new(cmd).args(args).output();

    if let Ok(output) = output {
        return output.stdout;
    }
    Vec::new()
}

pub fn spawn<K: Send + 'static>(
    args: cli::Args,
    mut cache: impl Cache<K> + Send + 'static,
    key: K,
) {
    match fork() {
        Ok(Fork::Parent(child)) => {
            debug!(
                "Continuing execution in parent process, new child has pid: {}",
                child
            );
        }
        Ok(Fork::Child) => {
            let args = args.get();
            if let Some((cmd, args)) = args.split_first() {
                let out = run(cmd, args);
                cache.patch(key, &out);
            }
        }
        Err(_) => error!("Fork failed"),
    }
}
