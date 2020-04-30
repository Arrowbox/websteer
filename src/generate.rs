use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

use anyhow::{anyhow, Result};
use dirs::data_dir;

pub fn generate_desktop(path: Option<PathBuf>) -> Result<()> {
    let mut desktop = File::create(
        path.unwrap_or(data_dir().unwrap().join("applications"))
            .join("WebSteer.desktop"),
    )?;

    write!(
        desktop,
"[Desktop Entry]
Version=0.1.0
Name=WebSteer
GenericName=Web Browser
Comment=Route urls to browsers
Exec={} open %U
StartupNotify=true
Terminal=false
Type=Application
Categories=Network;WebBrowser;
MimeType=x-scheme-handler/unknown;x-scheme-handler/about;text/html;text/xml;application/xhtml_xml;image/webp;x-scheme-handler/http;x-scheme-handler/https;x-scheme-handler/ftp;",
        std::env::current_exe()
            .unwrap_or(PathBuf::from("websteer"))
            .display()
    )
    .map_err(|x| anyhow!(x))
}
