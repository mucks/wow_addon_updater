use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Addon {
    pub download_url: String,
    pub file_name: String,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    pub path: PathBuf,
    pub added: Vec<Addon>,
    pub installed: Vec<Addon>,
}
