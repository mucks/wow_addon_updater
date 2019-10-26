use actix_web::HttpRequest;

pub fn get_config(_req: HttpRequest) -> String {
    serde_json::to_string(&crate::config::get().unwrap()).unwrap()
}

pub fn add_addon() {}
