#!/bin/bash

cd wow_addon_updater_webview
cargo web deploy
cd ..
cargo build --bin wow_addon_updater --release
yes | cp ./target/release/wow_addon_updater ~/.cargo/bin/