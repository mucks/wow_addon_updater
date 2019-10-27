use web_view::*;

pub fn run(port: u16) {
    // start web view in current thread
    // and point it to a port that was bound
    // to actix web server
    web_view::builder()
        .title("Actix webview example")
        .content(Content::Url(format!("http://127.0.0.1:{}", port)))
        .size(400, 400)
        .resizable(true)
        .debug(true)
        .user_data(())
        .invoke_handler(|_webview, _arg| Ok(()))
        .run()
        .unwrap();
}
