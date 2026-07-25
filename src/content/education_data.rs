use serde::Deserialize;

use crate::content::SectionDescription;

#[derive(Deserialize, Clone)]
pub struct EducationLocale {
    pub number: String,
    pub title: String,
    pub description: Option<SectionDescription>,
    pub entries: Vec<EducationEntry>,
    pub courses: Vec<Course>,
    pub centered: bool,
}
#[derive(PartialEq, Deserialize, Clone)]
pub struct EducationEntry {
    pub university: String,
    pub degree: String,
    pub years: String,
    pub description: String,
    pub highlights: Vec<String>,
    pub badge: String,
}

#[derive(Deserialize, Clone)]
pub struct Course {
    pub year: String,
    pub hours: String,
    pub title: String,
    pub provider: String,
    pub description: String,
    pub tags: Vec<String>,
}
