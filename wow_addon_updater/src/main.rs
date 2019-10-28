pub use wow_addon_updater_shared as shared;

mod api;
mod config;
mod err;
mod server;
mod ui;
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
        if unzip_and_save(&addon.addons_path(&conf.path), addon).is_ok() {
            new_conf.installed.push(addon.clone());
        }
    }
    config::save(&new_conf)?;
    Ok(())
}

fn unzip_and_save(path: &PathBuf, addon: &Addon) -> Result<(), err::Error> {
    let mut resp: reqwest::Response = reqwest::get(&addon.download_url)?;
    let mut buf = Vec::new();
    resp.read_to_end(&mut buf)?;
    let reader = io::Cursor::new(buf);
    let mut archive = zip::ZipArchive::new(reader).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let outpath = path.join(file.sanitized_name());
        println!("{}", file.name());

        if (&*file.name()).ends_with('/') {
            println!(
                "File {} extracted to \"{}\"",
                i,
                outpath.as_path().display()
            );
            fs::create_dir_all(&outpath).unwrap();
        } else {
            println!(
                "File {} extracted to \"{}\" ({} bytes)",
                i,
                outpath.as_path().display(),
                file.size()
            );
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

    Ok(())
}
