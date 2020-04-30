use anyhow::{anyhow, bail, Result};
use dirs;

use std::path;
use std::path::PathBuf;
use std::collections::HashMap;
use std::process::{Command, Stdio};
use serde::Deserialize;

use crate::config::{BrowserConfig, ExecConfig};

#[derive(Debug, PartialEq, Deserialize)]
#[serde(untagged)]
pub enum Executable<'cfg> {
    Desktop { desktop: &'cfg str),
    Exec {
        exec: &'cfg str,
        args: Option<Vec<&'cfg str>>,
    },
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Browser<'a> {
    pub name: &'a str,
    #[serde(flatten)]
    pub launcher: Executable<'a>,
}

impl<'a> Browser<'a> {
    fn args(&self) -> Vec<&str> {
        match &self.launcher {
            Executable::Desktop(d) => vec!["gtk-launch", d],
            Executable::Exec(e) => vec![e],
        }
    }

    fn run(&self, url: Option<&str>) -> Result<()> {

        let args = match &self.launcher {
            Executable::Desktop { desktop: d} => vec![d],
            Executable::Exec { exec: _, args: a } => a.unwrap_or(vec![]),
        }.extend(url.unwrap_or(vec![]))

        let status = Command::new(match &self.launcher {
            Executable::Desktop { desktop: _ } => "gtk-launch",
            Executable::Exec { exec: e } => e,
        })
            .args(args)
            .stdout(Stdio::null())
            .status()
            .expect("Failed to run command");

        if status.success() {
            Ok(())
        } else {
            Err(anyhow!("Command failed"))
        }
    }

    pub fn launch(&self) -> Result<()> {
        self.run(None)
    }

    pub fn open(&self, url: &str) -> Result<()> {
        self.run(Some(url))
    }
}

trait Launcher {
    pub fn launch(&self, url: &str) -> Result<()> {
        self.run(Some(vec![desktop]))
    }

    pub fn open(&self) -> Result<()> {
        self.run(None)
    }

    fn run(&self, args: Option<Vec<&str>>) -> Result<()> {
        let status = Command::new("dex")
            .args(args.unwrap_or(vec![]))
            .stdout(Stdio::null())
            .expect("Failed to run command");

        if status.success() {
            Ok(())
        } else {
            Err(anyhow!("Command failed"))
        }
    }

}

struct Dex;

impl Dex {
    fn find_desktop(&self, desktop: &str) -> Result<PathBuf> {
        vec![
            dirs::data_dir().unwrap().push("applications"), 
            PathBuf::from(r"/usr/local/share/applications"),
            PathBuf::from(r"/usr/share/applications")
        ].iter()
        .map(|p| p.join(desktop)) // Add in the desktop file
        .find(|d| path::exists(d))
        .ok_or(Err(anyhow!("Can't find desktop entry"))
    }

    pub fn launch(&self, desktop: &str) -> Result

    fn run(&self, args: Option<Vec<&str>>) {
        let status = Command::new("dex")
            .args(args.unwrap_or(vec![]))
            .stdout(Stdio::null())
            .expect("Failed to run command");

}

impl Launcher for Dex {
    pub fn launch(&self, desktop: &str) -> Result<PathBuf> {
        self.run(

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_browser_new() {
        assert_eq!(
            Browser::new("foo", Some("bar"), Some("/path/to/bin")).unwrap(),
            Browser {
                name: "foo",
                launcher: Executable::Desktop("bar"),
            }
        );

        assert_eq!(
            Browser::new("foo", None, Some("/path/to/bin")).unwrap(),
            Browser {
                name: "foo",
                launcher: Executable::Exec("/path/to/bin"),
            }
        );

        assert_eq!(
            Browser::new("foo", Some("bar"), None).unwrap(),
            Browser {
                name: "foo",
                launcher: Executable::Desktop("bar"),
            }
        );
    }

    #[test]
    fn test_browser_desktop() {
        assert_eq!(
            Browser::from_desktop("foo", "bar"),
            Browser {
                name: "foo",
                launcher: Executable::Desktop("bar"),
            }
        );
    }

    #[test]
    fn test_browser_exec() {
        assert_eq!(
            Browser::from_exec("foo", "/path/to/buzz"),
            Browser {
                name: "foo",
                launcher: Executable::Exec("/path/to/buzz"),
            }
        );
    }

    #[test]
    fn test_browser_args() {
        let browser = Browser {
            name: "foo",
            launcher: Executable::Desktop("bar"),
        };

        assert_eq!(browser.args(), vec!["gtk-launch", "bar"]);

        let browser = Browser {
            name: "fizz",
            launcher: Executable::Exec("/path/to/buzz"),
        };

        assert_eq!(browser.args(), vec!["/path/to/buzz"]);
    }
}
