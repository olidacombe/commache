use config::Config;
use derive_builder::Builder;
use lazy_static::lazy_static;
use std::env;
use std::path::PathBuf;

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
}

impl CommacheConfigBuilder {
    fn default_db_dir() -> PathBuf {
        APP_DIR.join("db")
    }
}

/// Gets app config from config files and the environment
pub fn get() -> CommacheConfig {
    let config = Config::builder()
        .add_source(
            config::Environment::with_prefix("COMMACHE")
                .try_parsing(true)
                .separator("_")
                .list_separator(" "),
        )
        .build()
        .unwrap();

    let config_builder: CommacheConfigBuilder = config.try_deserialize().unwrap();
    config_builder.build().unwrap()
}
