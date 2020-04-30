use std::collections::HashMap;
use std::fs::File;
use std::path::PathBuf;

use anyhow::{anyhow, Result};
use dirs::config_dir;
use serde::Deserialize;

use crate::browser::{Browser, Executable};

#[derive(Debug, Deserialize)]
pub struct RuleConfig {
    pub regex: String,
    pub browser: String,
    #[serde(default = "default_ambiguous")]
    pub ambiguous: bool,
}

fn default_ambiguous() -> bool {
    false
}

#[derive(Debug, Deserialize)]
pub struct Config<'cfg> {
    pub dialog: &'cfg str,
    pub default: &'cfg str,
    pub prompt: Vec<&'cfg str>,
    pub browser: HashMap<&'cfg str, Browser<'cfg>>,
    pub rule: Vec<RuleConfig>,
}

impl Config {
    pub fn from_yaml(yaml: &str) -> Result<Config> {
        let config: Config = serde_yaml::from_str(yaml).unwrap();
        Ok(config)
    }

    pub fn from_file(path: &Option<PathBuf>) -> Result<Config> {
        let default_path = config_dir().unwrap().join("websteer").join("config.yaml");

        serde_yaml::from_reader(File::open(match path {
            Some(p) => p,
            None => &default_path,
        })?)
        .map_err(|_| anyhow!("Failed to read"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse() {
        let config: Config = Config::from_yaml(
            r"
dialog: kdialog
default: personal
prompt:
    - personal
    - work

browser:
    personal:
        name: 'Personal Browser'
        desktop: personal-browser.desktop
    work:
        name: 'Work Browser'
        exec: '/usr/bin/personal-browser --various --args=foo'
    test:
        name: 'Test Browser'
        exec: '/usr/bin/other-browser'

rule:
    - regex: localhost
      browser: test
    - regex: workcorp
      browser: work
    - regex: 'drive\.google\.com'
      browser: work
      ambiguous: true
    - regex: 'facebook\.com'
      browser: personal
",
        )
        .unwrap();
        assert_eq!(config.dialog, "kdialog");
        assert_eq!(config.default, "personal");
        dbg!(config);
    }
}
