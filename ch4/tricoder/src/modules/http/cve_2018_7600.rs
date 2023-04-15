use crate::{
    modules::{HttpFinding, HttpModule, Module},
    Error,
};
use async_trait::async_trait;
use regex::Regex;
use reqwest::Client;

pub struct Cve2018_7600 {
    form_regex: Regex,
}

impl Cve2018_7600 {
    pub fn new() -> Self {
        Cve2018_7600 {
            form_regex: Regex::new(
                r#"<input type="hidden" name="form_build_id" value="([^"]+)" />"#,
            )
            .expect("http/cve_2018_7600: compiling regexp"),
        }
    }
}

impl Module {}
