use crate::runner::{Runner, RunnerImpl};
use crate::CONFIG;
use crate::{cache::Cache, cli::Args};
use fork::{daemon, Fork};
use ipc_channel::ipc::{IpcOneShotServer, IpcSender};
use serde::Serialize;
use tracing::{debug, error};

pub fn queue<R: Runner>(runner: R) {
    let (args, cache) = runner.mv();
    debug!("queue {:?}", &args);
    let sender = get_sender(cache);
    if let Err(e) = sender.send(args) {
        error!("failure sending job to daemon: {:?}", e);
    }
}

fn get_sender<T>(cache: impl Cache<Vec<u8>>) -> IpcSender<T>
where
    T: Serialize,
{
    match try_attach_sender() {
        Ok(sender) => {
            return sender;
        }
        Err(e) => {
            error!("first attach attempt: {:?}", e);
        }
    }

    spawn_server(cache);

    match try_attach_sender() {
        Err(e) => {
            error!(
                "failed to attach to existing daemon after spawn call: {:?}",
                &e
            );
            panic!(
                "failed to attach to existing daemon after spawn call: {:?}",
                e
            );
        }
        Ok(sender) => {
            debug!("attached");
            sender
        }
    }
}

fn try_attach_sender<T>() -> Result<IpcSender<T>, std::io::Error>
where
    T: Serialize,
{
    match CONFIG.sock_path() {
        Some(sock) => {
            debug!("attempting to attach {:?}", &sock);
            IpcSender::connect(sock)
        }
        None => Err(std::io::Error::from(std::io::ErrorKind::NotFound)),
    }
}

fn spawn_server(mut cache: impl Cache<Vec<u8>>) {
    debug!("spawn_server");
    let (server, sock) = match IpcOneShotServer::<Args>::new() {
        Ok((server, sock)) => (server, sock),
        Err(e) => {
            error!("failed to start ipc endpoint: {:?}", e);
            panic!("failed to start ipc endpoint: {:?}", e);
        }
    };
    CONFIG.write_sock(&sock);
    debug!("ABOUT TO FORK");
    // after we've written to the sock info file, we're happy to fork and let parent
    // return, so that second attempt to connect can be made
    if let Ok(Fork::Child) = daemon(true, true) {
        debug!("Entered daemon process");
        match server.accept() {
            Ok((rx, mut args)) => loop {
                let runner = RunnerImpl::new(args, cache);
                (_, cache) = runner.run_and_cache();

                loop {
                    match rx.recv() {
                        Ok(r) => {
                            args = r;
                            break;
                        }
                        Err(e) => {
                            error!("error recieving args: {:?}", e);
                        }
                    }
                }
            },
            Err(e) => {
                error!("error setting up args receiver: {:?}", e);
            }
        }
    }

    debug!("END spawn_server");
}
