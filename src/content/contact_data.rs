use serde::Deserialize;

use crate::content::SectionDescription;

#[derive(Deserialize, Clone)]
pub struct ContactLocale {
    pub number: String,
    pub title: String,
    pub description: Option<SectionDescription>,
    pub channels: Vec<ContactChannel>,
    pub centered: bool,
}

#[derive(Deserialize, Clone)]
pub struct ContactChannel {
    pub label: String,
    pub value: String,
    pub icon: String,
    pub url: Option<String>,
    pub description: String,
}
