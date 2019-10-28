use web_view::*;

pub fn run(port: u16) {
    web_view::builder()
        .title("Wow Addon Updater")
        .content(Content::Url(format!("http://127.0.0.1:{}", port)))
        .size(1280, 800)
        .resizable(true)
        .debug(true)
        .user_data(())
        .invoke_handler(|_webview, _arg| Ok(()))
        .run()
        .unwrap();
}
