pub use wow_addon_updater_shared as shared;

mod api;
mod client;
mod config;
mod err;
mod server;
mod ui;
mod wow_interface;

use futures::future::{lazy, Future};
use shared::{Addon, Config};
use std::fs;
use std::io;
use std::io::Read;
use std::path::PathBuf;

fn main() {
    server::start();
}

pub fn update() -> impl Future<Item = (), Error = ()> {
    config::update_added().map_err(|_| ()).and_then(|_| {
        let conf = config::get().unwrap();

        let addons = conf.added.iter().filter(|a| {
            match conf.installed.iter().find(|b| b.file_name == a.file_name) {
                Some(b) => a.version != b.version,
                None => true,
            }
        });

        let mut futs = Vec::new();

        for addon in addons {
            futs.push(unzip_and_save(addon.addons_path(&conf.path), addon.clone()));
        }

        let f = futures::future::join_all(futs);

        f.map_err(|_| ()).then(|x| {
            x.map_err(|_| ()).and_then(|addons| {
                println!("happens");
                let mut conf = config::get().unwrap();
                for addon in addons {
                    conf.installed.push(addon);
                }
                config::save(&conf).unwrap();
                Ok(())
            })
        })
    })
}

fn unzip_and_save(path: PathBuf, addon: Addon) -> impl Future<Item = Addon, Error = ()> {
    client::get_bytes(addon.download_url.clone())
        .map_err(|_| ())
        .and_then(move |bytes| {
            let reader = io::Cursor::new(bytes);
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
            Ok(addon)
        })
}
