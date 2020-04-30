use anyhow::{anyhow, Result};
use regex::Regex;

use crate::config::RuleConfig;

pub struct Rule<'a> {
    regex: &'a str,
    browser: &'a str,
    ambiguous: bool,
}

impl<'a> Rule<'a> {
    pub fn new(browser: &'a str, regex: &'a str, ambiguous: bool) -> Rule<'a> {
        Rule {
            regex,
            browser,
            ambiguous,
        }
    }

    pub fn compare(&self, url: &str) -> Result<(&'a str, bool)> {
        if Regex::new(self.regex)?.is_match(url) {
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
