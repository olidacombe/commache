use thiserror::Error;
use zeromq::ZmqError;

#[derive(Error, Debug)]
pub enum ZmqBinSerdeError {
    #[error("zmq error")]
    Zmq(#[from] ZmqError),

    #[error("bincode error")]
    Bincode(#[from] bincode::Error),
}
