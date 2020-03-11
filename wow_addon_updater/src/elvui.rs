use crate::shared::Addon;
use select::{
    document::Document,
    predicate::{Attr, Class},
};

pub fn get_addon(url: &str) -> Result<Option<Addon>, reqwest::Error> {
    let doc = crate::util::url_to_doc(url)?;
    Ok(doc_to_addon(url, &doc))
}

fn doc_to_addon(url: &str, doc: &Document) -> Option<Addon> {
    let download_url = format!(
        "https://www.tukui.org{}",
        doc.find(Class("btn-border-w")).next()?.attr("href")?,
    );

    let file_name = download_url.split("/downloads/").nth(1)?.trim();
    let version = file_name.replace("elvui-", "").replace(".zip", "");

    Some(Addon {
        url: url.into(),
        download_url: download_url.clone(),
        file_name: file_name.into(),
        version: version.into(),
        patch: "8".into(),
    })
}
