use daemonizr::{Daemonizr, DaemonizrError, Stderr, Stdout};
use std::path::PathBuf;
use thiserror::Error;
use zeromq::{PubSocket, Socket, SocketRecv, SocketSend, ZmqError};

use crate::cli::Args;
use crate::runner::Runner;
use crate::CONFIG;
use tracing::{debug, error};

pub async fn queue<R: Runner>(runner: R) -> Result<(), R::Error> {
    debug!("queue");

    {
        let mut socket = PubSocket::new();
        debug!("connecting publisher");
        if let Ok(_) = socket.connect(&CONFIG.sock_uri()).await {
            debug!("sending");
            match socket.send(runner.try_msg()?).await {
                Ok(_) => return Ok(()),
                Err(e) => error!("{:?}", e),
            }
        }
    }

    if let Err(e) = spawn_daemon(runner).await {
        error!("{:?}", e);
    }
    Ok(())
}

async fn spawn_daemon<R: Runner>(initial_runner: R) -> Result<(), ServerError> {
    debug!("spawn_server");

    debug!("ABOUT TO DAEMONIZE");

    Daemonizr::new()
        .pidfile(PathBuf::from("dmnzr.pid"))
        .stdout(Stdout::Redirect(PathBuf::from("dmnzr.out")))
        .stderr(Stderr::Redirect(PathBuf::from("dmnzr.err")))
        .umask(0o027)?
        .spawn()?;

    println!("DAEMONIZED");

    let (_, mut cache) = initial_runner.run_and_cache();

    println!("first caching done");

    let mut socket = zeromq::SubSocket::new();

    println!("binding");
    socket.bind(&CONFIG.sock_uri()).await?;

    println!("subscribing");
    socket.subscribe("").await?;
    println!("subscribed");

    loop {
        let recv = socket.recv().await?;
        let r = recv.get(0);
        if r.is_none() {
            continue;
        }
        if let Ok(args) = bincode::deserialize::<Args>(r.unwrap()) {
            dbg!(&args);
            let runner = R::new(args, cache);
            (_, cache) = runner.run_and_cache();
        }
    }
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
