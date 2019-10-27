use crate::err::Error;
use crate::shared::{Addon, Config};
use serde_json;
use std::fs::{self, File};

pub fn get() -> Result<Config, Error> {
    Ok(if fs::metadata("./config.json").is_ok() {
        let f = File::open("./config.json")?;
        serde_json::from_reader(f)?
    } else {
        Config {
            path: "/home/shnaky/Downloads".parse().unwrap(),
            added: vec![Addon::default()],
            installed: Vec::new(),
        }
    })
}

pub fn save(config: &Config) -> Result<(), Error> {
    let f = File::create("./config.json")?;
    serde_json::to_writer(f, config)?;
    Ok(())
}
