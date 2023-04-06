use crate::runner::Runner;
use crate::CONFIG;
use crate::{cache::Cache, cli::Args};
use fork::{daemon, Fork};
use ipc_channel::ipc::{IpcOneShotServer, IpcSender};
use serde::Serialize;

pub fn queue<R: Runner>(runner: R) {
    let sender = get_sender();
    sender.send(runner.args());
}

/// TODO:Attempt to connect a sender to an existing server,
/// fork a server if this fails
fn get_sender<T>() -> IpcSender<T>
where
    T: Serialize,
{
    if let Ok(sender) = try_attach_sender() {
        return sender;
    }

    spawn_server();

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

fn spawn_server() {
    if let Ok(Fork::Child) = daemon(true, false) {
        let (server, sock) = IpcOneShotServer::<Args>::new().unwrap();
        CONFIG.write_sock(&sock);
        // TODO
        std::thread::sleep(std::time::Duration::from_secs(600));
    }
}
