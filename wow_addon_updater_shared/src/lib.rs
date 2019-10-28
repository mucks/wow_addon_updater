use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Addon {
    pub url: String,
    pub download_url: String,
    pub file_name: String,
    pub version: String,
    pub patch: String,
}

impl Addon {
    fn patch_to_f32(&self) -> f32 {
        match format!("0.{}", self.patch.replace(".", "")).parse::<f32>() {
            Ok(u) => u,
            Err(err) => {
                println!("could not parse patch to f32\n{}", err);
                0.0
            }
        }
    }

    pub fn addons_path(&self, wow_path: &PathBuf) -> PathBuf {
        let path = if self.is_retail() {
            wow_path.join("_retail_")
        } else {
            wow_path.join("_classic_")
        };
        path.join("Interface/AddOns")
    }

    pub fn is_classic(&self) -> bool {
        let p = self.patch_to_f32();
        return p >= 0.1 && p <= 0.2;
    }
    pub fn is_retail(&self) -> bool {
        return self.patch_to_f32() >= 0.8;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    pub path: PathBuf,
    pub added: Vec<Addon>,
    pub installed: Vec<Addon>,
}
