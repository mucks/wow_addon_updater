use crate::config;
use crate::shared::Config;
use crate::wow_interface;
use actix_web::web::Json;
use actix_web::HttpRequest;
use futures::{Future, IntoFuture};

pub fn get_config(_req: HttpRequest) -> String {
    serde_json::to_string(&crate::config::get().unwrap()).unwrap()
}

pub fn add_addon(url: String) -> impl IntoFuture<Item = String, Error = ()> {
    wow_interface::get_addon(url.clone())
        .map_err(|_| ())
        .and_then(move |addon| {
            let mut conf = config::get().unwrap();
            if conf.added.iter().find(|a| a.url == url).is_none() {
                conf.added.push(addon.clone());
                config::save(&conf).unwrap();
            }
            Ok("".into())
        })
}

pub fn save_config(conf: Json<Config>) -> String {
    println!("{:?}", conf);
    config::save(&conf).unwrap();
    "".into()
}

pub fn update() -> impl IntoFuture<Item = (), Error = ()> {
    crate::update()
}
