use crate::api::*;
use actix_web::{body::Body, web, App, HttpRequest, HttpResponse, HttpServer};
use futures::future::Future;
use mime_guess::from_path;
use rust_embed::RustEmbed;
use std::{borrow::Cow, sync::mpsc, thread};

#[derive(RustEmbed)]
#[folder = "target/deploy"]
struct Asset;

fn assets(req: HttpRequest) -> HttpResponse {
    let path = if req.path() == "/" {
        // if there is no path, return default file
        "index.html"
    } else {
        // trim leading '/'
        &req.path()[1..]
    };

    // query the file from embedded asset with specified path
    match Asset::get(path) {
        Some(content) => {
            let body: Body = match content {
                Cow::Borrowed(bytes) => bytes.into(),
                Cow::Owned(bytes) => bytes.into(),
            };
            HttpResponse::Ok()
                .content_type(from_path(path).first_or_octet_stream().as_ref())
                .body(body)
        }
        None => HttpResponse::NotFound().body("404 Not Found"),
    }
}

pub fn start() {
    let (server_tx, server_rx) = mpsc::channel();
    let (port_tx, port_rx) = mpsc::channel();

    // start actix web server in separate thread
    thread::spawn(move || {
        let sys = actix_rt::System::new("actix-example");

        let server = HttpServer::new(|| {
            App::new()
                .route("/api/update", web::get().to(update))
                .route("/api/config", web::get().to(get_config))
                .route("/api/config", web::post().to(save_config))
                .route("/api/addon/add", web::post().to(add_addon))
                .route("/api/addon/delete", web::post().to(delete_addon))
                .route("*", web::get().to(assets))
        })
        .bind("127.0.0.1:0")
        .unwrap();
        // we specified the port to be 0,
        // meaning the operating system
        // will choose some available port
        // for us
        // get the first bound address' port,
        // so we know where to point webview at
        let port = server.addrs().first().unwrap().port();
        let server = server.start();
        let _ = port_tx.send(port);
        let _ = server_tx.send(server);
        let _ = sys.run();
    });
    let port = port_rx.recv().unwrap();
    let server = server_rx.recv().unwrap();

    crate::ui::run(port);

    // gracefully shutdown actix web server
    let _ = server.stop(true).wait();
}
