use actix_web::client::Client;
use futures::future::{lazy, Future};

pub fn get_str(url: String) -> impl Future<Item = String, Error = ()> {
    get_bytes(url)
        .map_err(|_| ())
        .and_then(|bytes| Ok(String::from_utf8_lossy(&bytes.to_vec()).to_string()))
}

pub fn get_bytes(url: String) -> impl Future<Item = Vec<u8>, Error = ()> {
    Client::new()
        .get(url)
        .send()
        .map_err(|_| ())
        .and_then(|mut resp| {
            resp.body()
                .map_err(|_| ())
                .and_then(move |bytes| Ok(bytes.to_vec()))
        })
}
