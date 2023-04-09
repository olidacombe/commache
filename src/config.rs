use config::Config;
use derive_builder::Builder;
use lazy_static::lazy_static;
use std::env;
use std::path::PathBuf;
use tracing::{debug, error};

lazy_static! {
    static ref APP_DIR: PathBuf = {
        let mut app_dir = env::var("HOME").map_or_else(|_| env::temp_dir(), PathBuf::from);
        app_dir.push(".local");
        app_dir.push(std::env!("CARGO_PKG_NAME"));
        app_dir
    };
}

#[derive(Debug, Builder)]
#[builder(derive(serde::Deserialize))]
pub struct CommacheConfig {
    #[builder(default = "Self::default_db_dir()")]
    pub db_dir: PathBuf,
    #[builder(default = "Self::default_sock_path_file()")]
    pub sock_path_file: PathBuf,
}

impl CommacheConfigBuilder {
    fn default_db_dir() -> PathBuf {
        APP_DIR.join("db")
    }
    fn default_sock_path_file() -> PathBuf {
        APP_DIR.join("sock.path")
    }
}

/// Gets app config from config files and the environment
pub fn get() -> CommacheConfig {
    let config = Config::builder()
        .add_source(config::Environment::with_prefix(
            &std::env!("CARGO_PKG_NAME").to_uppercase(),
        ))
        .build()
        .unwrap();

    let config_builder: CommacheConfigBuilder = config.try_deserialize().unwrap();
    config_builder.build().unwrap()
}

impl CommacheConfig {
    pub fn sock_path(&self) -> Option<String> {
        debug!("reading sock path from {:?}", &self.sock_path_file);
        std::fs::read_to_string(&self.sock_path_file).ok()
    }
    pub fn write_sock(&self, sock: &str) {
        if let Err(e) = std::fs::write(&self.sock_path_file, sock) {
            error!(
                "failed to write sock info: {:?} > {:?} :: {:?}",
                sock, &self.sock_path_file, e
            );
        }
    }
}
