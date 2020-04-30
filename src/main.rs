use anyhow::{bail, Result};
use std::collections::HashMap;
use std::path::PathBuf;
use structopt::StructOpt;

mod browser;
mod config;
mod dialog;
mod generate;
mod rule;

struct App<'a> {
    dialog: &'a str,
    default: &'a str,
    prompt: &'a Vec<String>,
    browsers: HashMap<String, browser::Browser<'a>>,
    rules: Vec<rule::Rule<'a>>,
}

impl<'a> App<'a> {
    fn from_config(conf: &'a config::Config) -> Result<App<'a>> {
        let app = App {
            dialog: &conf.dialog,
            default: &conf.default,
            prompt: &conf.prompt,
            browsers: browser::generate(&conf.browser),
            rules: rule::generate(&conf.rule),
        };

        if !app.browsers.contains_key(app.default) {
            bail!("Default browser not in browser list");
        }

        if app.prompt.iter().any(|x| !app.browsers.contains_key(x)) {
            bail!("Prompt contains browser not in the list");
        }

        Ok(app)
    }

    fn open(&self, url: Option<String>) -> Result<()> {
        match &url {
            Some(u) => {
                let (browser, amb) = self
                    .rules
                    .iter()
                    .find_map(|x| x.compare(&u).ok())
                    .unwrap_or((self.default, false));

                if !amb {
                    self.browsers[browser].open(u)
                } else {
                    self.browsers[&dialog::Kdialog::new(
                        "Select browser".to_string(),
                        format!("Choose a browser to open '{}'", &u),
                        self.prompt
                            .iter()
                            .map(|x| (x.as_str(), self.browsers[x].name))
                            .collect(),
                        self.prompt.iter().position(|x| x == &browser),
                    )
                    .show()?]
                        .open(u)
                }
            }
            None => self.browsers[self.default].launch(),
        }
    }
}

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(subcommand)]
    command: Command,
}

#[derive(Debug, StructOpt)]
enum Command {
    /// Open url it browser
    Open {
        /// Config file location
        #[structopt(short, long, parse(from_os_str))]
        config: Option<PathBuf>,
        /// Select dialog
        /// URL to open
        url: Option<String>,
    },

    /// Generate desktop file
    GenDesktop {
        /// Config file location
        #[structopt(parse(from_os_str))]
        path: Option<PathBuf>,
    },
}

fn main() -> Result<()> {
    let opt = Opt::from_args();

    match opt.command {
        Command::Open { config, url } => {
            let cfg = config::Config::from_file(&config)?;

            let app = App::from_config(&cfg)?;

            app.open(url)
        }
        Command::GenDesktop { path } => generate::generate_desktop(path),
    }
}
