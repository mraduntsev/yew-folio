use serde::Deserialize;

use crate::content::SectionDescription;

#[derive(Deserialize, Clone)]
pub struct AboutLocale {
    pub number: String,
    pub title: String,
    pub description: Option<SectionDescription>,
    pub principles: Vec<Principle>,
    pub skills: Vec<Skill>,
    pub centered: bool,
}

#[derive(Deserialize, Clone)]
pub struct Principle {
    pub icon: String,
    pub title: String,
    pub description: String,
}

#[derive(Deserialize, Clone)]
pub struct Skill {
    pub gradient: String,
    pub letter: String,
    pub title: String,
    pub technologies: String,
}
