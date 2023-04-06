use crate::cli::Args;
use crate::CONFIG;
use fork::{daemon, Fork};
use ipc_channel::ipc::{IpcOneShotServer, IpcSender};
use serde::Serialize;

pub fn queue(args: Args) {
    let (server, name) = IpcOneShotServer::<Args>::new().unwrap();

    let tx = IpcSender::connect(name).unwrap();
    tx.send(args).unwrap();
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
    let (server, sock) = IpcOneShotServer::<Args>::new().unwrap();
    CONFIG.write_sock(&sock);
    if let Ok(Fork::Child) = daemon(true, false) {
        // TODO
    }
}
