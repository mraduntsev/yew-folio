use serde::Deserialize;

use crate::content::SectionDescription;

#[derive(Deserialize, Clone)]
pub struct ProjectsLocale {
    pub number: String,
    pub title: String,
    pub description: Option<SectionDescription>,
    pub projects: Vec<Project>,
    pub centered: bool,
}

#[derive(Deserialize, Clone)]
pub struct Project {
    pub title: String,
    pub description: String,
    pub tags: Vec<String>,
    pub gradient: String,
    pub version: String,
    pub stars: u32,
    pub forks: u32,
    pub featured: bool,
    pub repo_url: Option<String>,
}
