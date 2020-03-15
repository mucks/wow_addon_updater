pub use wow_addon_updater_shared as shared;

mod api;
mod config;
mod elvui;
mod err;
mod server;
mod ui;
mod util;
mod wow_interface;

use shared::{Addon, Config};
use std::fs;
use std::io;
use std::io::Read;
use std::path::PathBuf;

fn main() {
    server::start();
}

pub fn update() -> Result<(), err::Error> {
    config::update_added()?;
    let conf = config::get()?;

    let addons = conf.added.iter().filter(|a| {
        match conf.installed.iter().find(|b| b.file_name == a.file_name) {
            Some(b) => a.version != b.version,
            None => true,
        }
    });

    let mut new_conf = conf.clone();

    for addon in addons {
        if let Ok(downloaded_addon) = unzip_and_save(&addon.addons_path(&conf.path), addon.clone())
        {
            new_conf.installed.push(downloaded_addon.clone());
        }
    }
    config::save(&new_conf)?;
    Ok(())
}

fn unzip_and_save(path: &PathBuf, mut addon: Addon) -> Result<Addon, err::Error> {
    let mut resp: reqwest::Response = reqwest::get(&addon.download_url)?;
    let mut buf = Vec::new();
    resp.read_to_end(&mut buf)?;
    let reader = io::Cursor::new(buf);
    let mut archive = zip::ZipArchive::new(reader).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let outpath = path.join(file.sanitized_name());
        let file_path: PathBuf = file.name().parse().unwrap();
        let addon_dir_path = file_path.iter().next().unwrap().to_str().unwrap();
        addon.dir_paths.push(addon_dir_path.into());

        if (&*file.name()).ends_with('/') {
            fs::create_dir_all(&outpath).unwrap();
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p).unwrap();
                }
            }
            let mut outfile = fs::File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap();
            }
        }
    }
    addon.dir_paths.dedup();

    Ok(addon)
}
