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
    conf.added
        .push(wow_interface::get_addon(&url).unwrap().unwrap());
    config::save(&conf);
    "".into()
}

pub fn save_config(conf: Json<Config>) -> String {
    config::save(&conf).unwrap();
    "".into()
}
