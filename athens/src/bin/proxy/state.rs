use std::{
    env,
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    path::PathBuf,
};

use color_eyre::eyre::{Context, Result};
use directories::ProjectDirs;
use ed25519_dalek::{SigningKey, VerifyingKey};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct State {
    pub user_id: String,
    pub user_key: SigningKey,
    pub device_pub_keys: Vec<VerifyingKey>,
}

const CONFIG_FILE_NAME: &str = "config.toml";
const MSG_FILE_NAME: &str = "msgs.toml";

lazy_static! {
    pub static ref PROJECT_NAME: String = env!("CARGO_CRATE_NAME").to_uppercase().to_string();
    pub static ref DATA_FOLDER: Option<PathBuf> =
        env::var(format!("{}_DATA", PROJECT_NAME.clone()))
            .ok()
            .map(PathBuf::from);
    pub static ref CONFIG_FOLDER: Option<PathBuf> =
        env::var(format!("{}_CONFIG", PROJECT_NAME.clone()))
            .ok()
            .map(PathBuf::from);
}

impl State {
    pub fn new(user_id: String, signing_key: SigningKey) -> Result<Self> {
        let config_dir = get_config_dir();

        fs::create_dir_all(&config_dir)
            .with_context(|| format!("Failed to create config directory: {:?}", config_dir))?;

        let mut f = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(config_dir.join("config.toml"))
            .with_context(|| {
                format!(
                    "Error occurred when writing to configuration file: {:?}.",
                    config_dir.join(CONFIG_FILE_NAME)
                )
            })?;

        let config = Self {
            user_id,
            user_key: signing_key,
            device_pub_keys: Vec::new(),
        };

        f.write_all(toml::to_string(&config)?.as_bytes())?;

        Ok(config)
    }

    pub fn read() -> Result<Self> {
        let mut f = File::open(get_config_dir().join("config.toml"))?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;

        let cfg: Self = toml::from_str(s.as_str())?;
        Ok(cfg)
    }

    pub fn add_device_pub_key(mut self, key: VerifyingKey) -> Result<Self> {
        let config_dir = get_config_dir();
        let mut f = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(config_dir.join("config.toml"))
            .with_context(|| {
                format!(
                    "Error occurred when writing to configuration file: {:?}.",
                    config_dir.join(CONFIG_FILE_NAME)
                )
            })?;

        self.device_pub_keys.push(key);

        f.write_all(toml::to_string(&self)?.as_bytes())?;

        Ok(self.to_owned())
    }
}

pub fn get_config_dir() -> PathBuf {
    if let Some(s) = CONFIG_FOLDER.clone() {
        s
    } else if let Some(proj_dirs) = project_directory() {
        proj_dirs.config_local_dir().to_path_buf()
    } else {
        PathBuf::from(".").join(".config")
    }
}

pub fn get_data_dir() -> PathBuf {
    if let Some(s) = DATA_FOLDER.clone() {
        s
    } else if let Some(proj_dirs) = project_directory() {
        proj_dirs.data_local_dir().to_path_buf()
    } else {
        PathBuf::from(".").join(".config")
    }
}

pub fn get_outfile() -> PathBuf {
    get_data_dir().join(MSG_FILE_NAME)
}

fn project_directory() -> Option<ProjectDirs> {
    ProjectDirs::from("com", "suri", env!("CARGO_PKG_NAME"))
}
