use crate::client;
use crate::shared::Addon;
use futures::future::{lazy, Future};
use select::document::Document;
use select::predicate::{Attr, Class, Name, Predicate};

pub fn get_addon(url: String) -> impl Future<Item = Addon, Error = ()> {
    let download_url = url
        .replace("/downloads/info", "/downloads/download")
        .replace(".html", "");

    client::get_str(url.clone())
        .map_err(|_| ())
        .and_then(move |url_content| {
            client::get_str(download_url.clone())
                .map_err(|_| ())
                .and_then(move |download_content| {
                    let doc = Document::from(url_content.as_str());
                    let download_doc = Document::from(download_content.as_str());

                    let (version, patch) = get_version_and_patch(&doc).unwrap();
                    let (download_url, file_name) =
                        get_download_url_and_file_name(&download_doc).unwrap();

                    Ok(Addon {
                        url: url,
                        download_url: download_url,
                        file_name: file_name,
                        version: version,
                        patch: patch,
                    })
                })
        })
}

fn get_version_and_patch(doc: &Document) -> Option<(String, String)> {
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

    Some((version, patch.into()))
}

fn get_download_url_and_file_name(doc: &Document) -> Option<(String, String)> {
    let download_url = doc
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

    Some((download_url.into(), file_name.into()))
}
