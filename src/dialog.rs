use anyhow::Result;

use std::process::Command;

pub struct Kdialog {
    exec: String,
    args: Vec<String>,
}

impl Kdialog {
    pub fn new(
        title: String,
        description: String,
        choices: Vec<(&str, &str)>,
        default: Option<usize>,
    ) -> Kdialog {
        let mut args: Vec<String> = Vec::new();
        for (i, (browser, name)) in choices.iter().enumerate() {
            args.push(browser.to_string());
            args.push(name.to_string());
            args.push(
                if i == match default {
                    Some(def) => def,
                    None => 0,
                } {
                    "on".to_string()
                } else {
                    "off".to_string()
                },
            );
        }

        Kdialog {
            exec: "kdialog".to_string(),
            args: vec![
                "--title".to_string(),
                title.to_string(),
                "--radiolist".to_string(),
                description.to_string(),
            ]
            .into_iter()
            .chain(args.into_iter())
            .collect(),
        }
    }

    pub fn show(&self) -> Result<String> {
        let output = Command::new(&self.exec)
            .args(&self.args)
            .output()
            .expect("Failed to run command");
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    }
}
