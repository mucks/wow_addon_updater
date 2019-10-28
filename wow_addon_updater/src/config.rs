use crate::err::Error;
use crate::shared::{Addon, Config};
use serde_json;
use std::fs::{self, File};
use std::path::PathBuf;

pub fn get() -> Result<Config, Error> {
    Ok(if fs::metadata("./config.json").is_ok() {
        let f = File::open("./config.json")?;
        serde_json::from_reader(f)?
    } else {
        let default_path: PathBuf = if cfg!(windows) {
            "C:/Program Files (x86)/World of Warcraft".parse()?
        } else if cfg!(unix) {
            let p: PathBuf = std::env::var("HOME")?.parse()?;
            p.join("Games/battlenet/drive_c/Program Files (x86)/World of Warcraft")
        } else {
            "/".parse()?
        };
        Config {
            path: default_path,
            added: Vec::new(),
            installed: Vec::new(),
        }
    })
}

pub fn save(config: &Config) -> Result<(), Error> {
    let f = File::create("./config.json")?;
    serde_json::to_writer(f, config)?;
    Ok(())
}

pub fn update_added() -> Result<(), Error> {
    let mut conf = get()?;
    for addon in conf.added.iter_mut() {
        if let Some(new_addon) = crate::wow_interface::get_addon(&addon.url)? {
            addon.version = new_addon.version;
        }
    }
    Ok(save(&conf)?)
}
