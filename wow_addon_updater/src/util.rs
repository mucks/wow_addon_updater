use select::document::Document;

pub fn url_to_doc(url: &str) -> Result<Document, reqwest::Error> {
    let html = reqwest::get(url)?.text()?;
    Ok(Document::from(html.as_str()))
}
