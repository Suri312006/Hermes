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
    pub signing_key: SigningKey,
}

const CLI_STATE_FILE_NAME: &str = "cli_state.toml";

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
    pub fn new(signing_key: SigningKey) -> Result<Self> {
        let config_dir = get_config_dir();

        fs::create_dir_all(&config_dir)
            .with_context(|| format!("Failed to create config directory: {:?}", config_dir))?;

        let mut f = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(config_dir.join(CLI_STATE_FILE_NAME))
            .with_context(|| {
                format!(
                    "Error occurred when writing to configuration file: {:?}.",
                    config_dir.join(CLI_STATE_FILE_NAME)
                )
            })?;

        let config = Self { signing_key };

        f.write_all(toml::to_string(&config)?.as_bytes())?;

        Ok(config)
    }

    pub fn read() -> Result<Self> {
        let mut f = File::open(get_config_dir().join(CLI_STATE_FILE_NAME))?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;

        let cfg: Self = toml::from_str(s.as_str())?;
        Ok(cfg)
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

fn project_directory() -> Option<ProjectDirs> {
    ProjectDirs::from("com", "suri", env!("CARGO_PKG_NAME"))
}
