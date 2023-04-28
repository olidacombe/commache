use daemonizr::{Daemonizr, DaemonizrError, Stderr, Stdout};
use std::path::PathBuf;
use std::process::exit;
use thiserror::Error;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};
use zeromq::{PubSocket, Socket, SocketRecv, SocketSend, ZmqError};

use crate::runner::{Runner, RunnerImpl};
use crate::CONFIG;
use crate::{cache::Cache, cli::Args};
use serde::Serialize;
use tracing::{debug, error, info};

pub async fn queue<R: Runner>(runner: R) -> Result<(), R::Error> {
    // let (args, cache) = runner.mv();
    // debug!("queue {:?}", &args);

    let mut socket = PubSocket::new();
    if let Ok(_) = socket.connect(&CONFIG.sock_uri()).await {
        match socket.send(runner.try_msg()?).await {
            Ok(_) => return Ok(()),
            Err(e) => error!("{:?}", e),
        }
    }

    if let Err(e) = spawn_daemon(runner) {
        error!("{:?}", e);
    }
    Ok(())
    // let sender = get_sender(cache);
    // if let Err(e) = sender.send(args) {
    //     error!("failure sending job to daemon: {:?}", e);
    // }
    // Ok(())
}

fn spawn_daemon<R: Runner>(initial_runner: R) -> Result<(), ServerError> {
    debug!("spawn_server");

    // CONFIG.write_sock(&sock);
    debug!("ABOUT TO DAEMONIZE");
    // after we've written to the sock info file, we're happy to fork and let parent
    // return, so that second attempt to connect can be made

    Daemonizr::new()
        .pidfile(PathBuf::from("dmnzr.pid"))
        .stdout(Stdout::Redirect(PathBuf::from("dmnzr.out")))
        .stderr(Stderr::Redirect(PathBuf::from("dmnzr.err")))
        .umask(0o027)?
        .spawn()?;

    initial_runner.run_and_cache();

    Ok(())
}

// TODO try to start daemon, which should fail quickly when it's running
// then sendo to running daemon...
// fn get_sender<T>(cache: impl Cache<Vec<u8>>) -> IpcSender<T>
// where
//     T: Serialize,
// {
//     match try_attach_sender() {
//         Ok(sender) => {
//             debug!("attached sender");
//             return sender;
//         }
//         Err(e) => {
//             error!("first attach attempt: {:?}", e);
//         }
//     }
//
//     spawn_server(cache);
//
//     // loop {
//     match try_attach_sender() {
//         Err(e) => {
//             error!(
//                 "failed to attach to existing daemon after spawn call: {:?}",
//                 &e
//             );
//             panic!(
//                 "failed to attach to existing daemon after spawn call: {:?}",
//                 e
//             );
//         }
//         Ok(sender) => {
//             debug!("attached");
//             return sender;
//         }
//     }
//     // std::thread::sleep(std::time::Duration::from_secs(1));
//     // }
// }

// fn try_attach_sender<T>() -> Result<IpcSender<T>, std::io::Error>
// where
//     T: Serialize,
// {
//     match CONFIG.sock_path() {
//         Some(sock) => {
//             debug!("attempting to attach {:?}", &sock);
//             IpcSender::connect(sock)
//         }
//         None => Err(std::io::Error::from(std::io::ErrorKind::NotFound)),
//     }
// }
//

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("daemonize error")]
    Daemonizr(#[from] DaemonizrError),
}
