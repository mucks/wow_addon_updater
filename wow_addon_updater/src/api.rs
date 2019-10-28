use crate::config;
use crate::shared::Config;
use crate::wow_interface;
use actix_web::web::Json;
use actix_web::HttpRequest;

pub fn get_config(_req: HttpRequest) -> String {
    serde_json::to_string(&crate::config::get().unwrap()).unwrap()
}

pub fn add_addon(url: String) -> String {
    let mut conf = config::get().unwrap();
    if conf.added.iter().find(|a| a.url == url).is_none() {
        conf.added
            .push(wow_interface::get_addon(&url).unwrap().unwrap());
        config::save(&conf).unwrap();
    }
    "".into()
}

pub fn save_config(conf: Json<Config>) -> String {
    println!("{:?}", conf);
    config::save(&conf).unwrap();
    "".into()
}

pub fn update() -> String {
    crate::update();
    "".into()
}
