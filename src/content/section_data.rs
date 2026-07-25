use serde::Deserialize;
use yew::prelude::*;

#[derive(Clone, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum SectionDescription {
    Text(String),
    Link { label: String, href: String },
}

impl From<String> for SectionDescription {
    fn from(s: String) -> Self {
        SectionDescription::Text(s.clone())
    }
}

impl From<(String, String)> for SectionDescription {
    fn from(tuple: (String, String)) -> Self {
        SectionDescription::Link {
            label: tuple.0.clone(),
            href: tuple.1.clone(),
        }
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct SectionHeader {
    pub number: String,
    pub title: String,
    pub description: Option<SectionDescription>,
    pub centered: bool,
}
