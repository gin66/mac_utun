// VpnCloud - Peer-to-Peer VPN
// Copyright (C) 2015-2017  Dennis Schwerdel
// This software is licensed under GPL-3 or newer (see LICENSE.md)

extern crate cc;

fn main() {
    if cfg!(target_os = "macos") {
        cc::Build::new()
                .file("src/c/utun.c")
                .compile("utun");
    }
}
