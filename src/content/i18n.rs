use serde::Deserialize;
use serde_path_to_error::deserialize;

use crate::{
    content::{AboutLocale, ContactLocale, EducationLocale, HeroLocale, NavLocale, ProjectsLocale},
    hooks::Language,
};

#[derive(Deserialize, Clone)]
pub struct Locale {
    pub navbar: NavLocale,
    pub hero: HeroLocale,
    pub about: AboutLocale,
    pub education: EducationLocale,
    pub projects: ProjectsLocale,
    pub contact: ContactLocale,
}

pub fn load_locale(lang: Language) -> Locale {
    let json = match lang {
        Language::En => include_str!("locales/en.json"),
        Language::Ru => include_str!("locales/ru.json"),
    };

    let jd = &mut serde_json::Deserializer::from_str(json);
    let result: Result<Locale, _> = deserialize(jd);
    match result {
        Ok(locale) => locale,
        Err(e) => {
            let path = e.path().to_string();
            panic!("Invalid locale JSON at {}: {}", path, e);
        }
    }
}
