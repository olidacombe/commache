use daemonizr::{Daemonizr, DaemonizrError, Stderr, Stdout};
use std::path::PathBuf;
use thiserror::Error;
use zeromq::{PubSocket, Socket, SocketRecv, SocketSend, ZmqError};

use crate::cli::Args;
use crate::runner::Runner;
use crate::CONFIG;
use tracing::{debug, error};

pub fn queue<R: Runner>(runner: R) -> Result<(), R::Error> {
    debug!("queue");
    debug!("ABOUT TO DAEMONIZE");

    Daemonizr::new()
        // .pidfile(PathBuf::from("dmnzr.pid"))
        // .stdout(Stdout::Redirect(PathBuf::from("dmnzr.out")))
        // .stderr(Stderr::Redirect(PathBuf::from("dmnzr.err")))
        // .umask(0o027)?
        .spawn()?;

    println!("DAEMONIZED");

    let (_, mut cache) = initial_runner.run_and_cache();

    Ok(())
}

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("daemonize error")]
    Daemonizr(#[from] DaemonizrError),

    #[error("zmq error")]
    Zmq(#[from] ZmqError),

    #[error("bincode error")]
    Bincode(#[from] bincode::Error),
}
