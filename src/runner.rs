use crate::error::ZmqBinSerdeError;
use daemonizr::{Daemonizr, DaemonizrError};
use std::{process::Command, string::FromUtf8Error};

use tracing::{debug, error};
use zeromq::ZmqMessage;

use crate::{cache::Cache, cli::Args, key::ToKey};

pub trait Runner {
    type Cache: Cache<Vec<u8>>;
    type Error;

    fn new(args: Args, cache: Self::Cache) -> Self;
    fn get<F: Fn(String)>(self, cb: F) -> Result<(), Self::Error>;
    fn run_and_cache(self, background: bool) -> Result<(Vec<u8>, Self::Cache), Self::Error>;
}

pub struct RunnerImpl<K: Send + 'static, C: Cache<K>> {
    args: Args,
    cache: C,
    key: K,
}

fn run(cmd: &str, args: &[&str]) -> Option<Vec<u8>> {
    Command::new(cmd).args(args).output().map(|o| o.stdout).ok()
}

use thiserror::Error;

#[derive(Error, Debug)]
pub enum RunnerImplError {
    #[error("utf8 decode")]
    Utf8(#[from] FromUtf8Error),
    #[error("daemonize")]
    Daemonize(#[from] DaemonizrError),
}

impl<C> Runner for RunnerImpl<Vec<u8>, C>
where
    C: Cache<Vec<u8>>,
{
    type Cache = C;
    type Error = RunnerImplError;

    fn new(args: Args, cache: C) -> Self {
        let key = args.get().key();
        Self { args, cache, key }
    }

    fn get<F: Fn(String)>(self, cb: F) -> Result<(), Self::Error> {
        let v = self.cache.get(&self.key);
        if let Some(v) = v {
            debug!("cache hit");
            cb(v);
            // refresh cache in the background
            self.run_and_cache(true)?;
        } else {
            debug!("cache miss");
            let (v, _) = self.run_and_cache(false)?;
            let v = String::from_utf8(v)?;
            cb(v);
        }
        Ok(())
    }

    fn run_and_cache(mut self, background: bool) -> Result<(Vec<u8>, C), Self::Error> {
        if background {
            Daemonizr::new().spawn()?;
        }
        let args = self.args.get();
        if let Some((cmd, args)) = args.split_first() {
            if let Some(out) = run(cmd, args) {
                self.cache.patch(&self.key, &out);
                return Ok((out, self.cache));
            }
        }
        Ok((Vec::new(), self.cache))
    }
}
