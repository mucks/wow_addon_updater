use crate::{config, elvui, shared::Config, wow_interface};
use actix_web::web::Json;
use actix_web::HttpRequest;

pub fn get_config(_req: HttpRequest) -> String {
    serde_json::to_string(&crate::config::get().unwrap()).unwrap()
}

pub fn add_addon(url: String) -> String {
    let mut conf = config::get().unwrap();
    if conf.added.iter().find(|a| a.url == url).is_none() {
        let addon = if url.contains("tukui.org") {
            elvui::get_addon(&url).unwrap().unwrap()
        } else {
            wow_interface::get_addon(&url).unwrap().unwrap()
        };

        conf.added.push(addon);
        config::save(&conf).unwrap();
    }
    "".into()
}

use std::fs;

pub fn delete_addon(url: String) -> String {
    let mut conf = config::get().unwrap();
    if let Some(index) = conf.installed.iter().position(|a| a.url == url) {
        println!("{}", index);
        if let Some(addon) = conf.installed.get(index) {
            for path in &addon.dir_paths {
                let full_path = addon.addons_path(&conf.path).join(path);
                println!("{}", full_path.to_str().unwrap());
                fs::remove_dir_all(full_path).unwrap();
            }
            conf.installed.remove(index);
            conf.added.remove(index);
        }
    }
    config::save(&conf).unwrap();
    "".into()
}

pub fn save_config(conf: Json<Config>) -> String {
    println!("{:?}", conf);
    config::save(&conf).unwrap();
    "".into()
}

pub fn update() -> String {
    println!("{:?}", crate::update());
    "".into()
}
