use crate::shared::Addon;
use select::document::Document;
use select::predicate::{Attr, Class, Name, Predicate};

pub fn get_addon(url: &str) -> Result<Option<Addon>, reqwest::Error> {
    let download_url = url
        .replace("/downloads/info", "/downloads/download")
        .replace(".html", "");

    let dl_doc = url_to_doc(&download_url)?;
    let doc = url_to_doc(url)?;

    Ok(docs_to_addon(url, &doc, &dl_doc))
}

fn url_to_doc(url: &str) -> Result<Document, reqwest::Error> {
    let html = reqwest::get(url)?.text()?;
    Ok(Document::from(html.as_str()))
}

fn docs_to_addon(url: &str, doc: &Document, download_doc: &Document) -> Option<Addon> {
    let version = doc
        .find(Attr("id", "version"))
        .next()?
        .text()
        .replace("Version: ", "");

    let patch_text = doc
        .find(Attr("id", "screen-info"))
        .next()?
        .find(Class("alt1"))
        .nth(1)?
        .find(Name("div"))
        .next()?
        .text();
    let patch = patch_text.split("(").nth(1)?.split(")").next()?;

    let download_url = download_doc
        .find(Attr("id", "downloadLanding"))
        .next()?
        .find(Class("manuallink"))
        .next()?
        .find(Name("a"))
        .next()?
        .attr("href")?;

    let file_name = download_url
        .split("/downloads/")
        .nth(1)?
        .split("/")
        .nth(1)?
        .split("?")
        .next()?;

    Some(Addon {
        url: url.into(),
        download_url: download_url.into(),
        file_name: file_name.into(),
        version: version,
        patch: patch.into(),
    })
}
