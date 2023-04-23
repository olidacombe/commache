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
    #[builder(default = "Self::default_pid_file()")]
    pub pid_file: PathBuf,
    #[builder(default = "Self::default_sock_path()")]
    pub sock_path: PathBuf,
}

macro_rules! local_file {
    ($suffix:expr) => {
        APP_DIR.join(format!(
            "{}{}",
            &std::env!("CARGO_PKG_NAME").to_lowercase(),
            $suffix
        ))
    };
}

impl CommacheConfigBuilder {
    fn default_db_dir() -> PathBuf {
        local_file!(".db")
    }
    fn default_pid_file() -> PathBuf {
        local_file!(".pid")
    }
    fn default_sock_path() -> PathBuf {
        local_file!(".sock")
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
