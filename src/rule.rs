use anyhow::{anyhow, Result};
use regex::Regex;

use crate::config::RuleConfig;

pub struct Rule<'a> {
    regex: Regex,
    browser: &'a str,
    ambiguous: bool,
}

impl<'a> Rule<'a> {
    pub fn new(browser: &'a str, regex: &str, ambiguous: bool) -> Rule<'a> {
        Rule {
            regex: Regex::new(regex).unwrap(),
            browser,
            ambiguous,
        }
    }

    pub fn compare(&self, url: &str) -> Result<(&'a str, bool)> {
        if self.regex.is_match(url) {
            Ok((self.browser, self.ambiguous))
        } else {
            Err(anyhow!("No match"))
        }
    }
}

pub fn generate<'a>(rules: &'a Vec<RuleConfig>) -> Vec<Rule<'a>> {
    rules
        .iter()
        .map(|rc| Rule::new(&rc.browser, &rc.regex, rc.ambiguous))
        .collect()
}
