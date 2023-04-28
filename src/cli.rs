use crate::error::ZmqBinSerdeError;
use clap::Parser;
use serde::{Deserialize, Serialize};
use zeromq::{ZmqError, ZmqMessage};

#[derive(Debug, Parser, Deserialize, Serialize)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    cmd: Vec<String>,
}

impl Args {
    pub fn get(&self) -> Vec<&str> {
        self.cmd.iter().map(|a| a.as_str()).collect()
    }
}

impl TryFrom<ZmqMessage> for Args {
    type Error = ZmqBinSerdeError;

    fn try_from(msg: ZmqMessage) -> Result<Self, Self::Error> {
        let m = msg.get(0).ok_or(ZmqError::NoMessage)?;
        Ok(bincode::deserialize::<Self>(m)?)
    }
}

impl TryFrom<&Args> for ZmqMessage {
    type Error = ZmqBinSerdeError;

    fn try_from(args: &Args) -> Result<Self, Self::Error> {
        let b = bincode::serialize(args)?;
        Ok(ZmqMessage::from(b))
    }
}
