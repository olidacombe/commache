use crate::runner::{Runner, RunnerImpl};
use crate::CONFIG;
use crate::{cache::Cache, cli::Args};
use fork::{daemon, Fork};
use ipc_channel::ipc::{IpcOneShotServer, IpcSender};
use serde::Serialize;

pub fn queue<R: Runner>(runner: R) {
    let (args, cache) = runner.mv();
    let sender = get_sender(cache);
    sender.send(args).expect("Error queuing refresh");
}

fn get_sender<T>(cache: impl Cache<Vec<u8>>) -> IpcSender<T>
where
    T: Serialize,
{
    if let Ok(sender) = try_attach_sender() {
        return sender;
    }

    spawn_server(cache);

    try_attach_sender().unwrap()
}

fn try_attach_sender<T>() -> Result<IpcSender<T>, std::io::Error>
where
    T: Serialize,
{
    match CONFIG.sock_path() {
        Some(sock) => IpcSender::connect(sock),
        None => Err(std::io::Error::from(std::io::ErrorKind::NotFound)),
    }
}

fn spawn_server(mut cache: impl Cache<Vec<u8>>) {
    let (server, sock) = IpcOneShotServer::<Args>::new().unwrap();
    CONFIG.write_sock(&sock);
    if let Ok(Fork::Child) = daemon(true, false) {
        if let Ok((rx, mut args)) = server.accept() {
            loop {
                let runner = RunnerImpl::new(args, cache);
                (_, cache) = runner.run_and_cache();
                // TODO try harder
                args = rx.recv().unwrap();
            }
        }
    }
}
