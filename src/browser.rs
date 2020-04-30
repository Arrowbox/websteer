use anyhow::{anyhow, bail, Result};

use std::collections::HashMap;
use std::process::{Command, Stdio};

use crate::config::{BrowserConfig, ExecConfig};

#[derive(Debug, PartialEq)]
enum Executable<'a> {
    Desktop(&'a str),
    Exec(&'a str),
}

#[derive(Debug, PartialEq)]
pub struct Browser<'a> {
    pub name: &'a str,
    launcher: Executable<'a>,
}

impl<'a> Browser<'a> {
    pub fn new(
        name: &'a str,
        desktop: Option<&'a str>,
        exec: Option<&'a str>,
    ) -> Result<Browser<'a>> {
        let launcher = if let Some(d) = desktop {
            Executable::Desktop(d)
        } else if let Some(e) = exec {
            Executable::Exec(e)
        } else {
            bail!("Must have desktop or exec")
        };

        Ok(Browser { name, launcher })
    }

    pub fn from_desktop(name: &'a str, desktop: &'a str) -> Browser<'a> {
        Browser::new(name, Some(desktop), None).unwrap()
    }

    pub fn from_exec(name: &'a str, exec: &'a str) -> Browser<'a> {
        Browser::new(name, None, Some(exec)).unwrap()
    }

    pub fn from_config(conf: &'a BrowserConfig) -> Browser<'a> {
        Browser {
            name: &conf.name,
            launcher: match &conf.launcher {
                ExecConfig::Desktop(d) => Executable::Desktop(&d),
                ExecConfig::Exec(d) => Executable::Exec(&d),
            },
        }
    }

    fn args(&self) -> Vec<&str> {
        match &self.launcher {
            Executable::Desktop(d) => vec!["gtk-launch", d],
            Executable::Exec(e) => vec![e],
        }
    }

    fn run(&self, url: Option<&str>) -> Result<()> {
        let mut args = self.args();

        if let Some(u) = url {
            args.push(u);
        }

        let status = Command::new(&args[0])
            .args(&args[1..])
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

pub fn generate<'a>(browsers: &'a HashMap<String, BrowserConfig>) -> HashMap<String, Browser<'a>> {
    let mut bmap: HashMap<String, Browser<'a>> = HashMap::new();
    browsers.iter().for_each(|(key, bc)| {
        bmap.insert(key.to_owned(), Browser::from_config(&bc));
    });

    bmap
}

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
