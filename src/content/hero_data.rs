use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct HeroLocale {
    pub buttons: HeroButtons,
    pub greeting: String,
    pub name: String,
    pub build_intro: String,
    pub typed_phrases: Vec<String>,
    pub role: String,
    pub skills: Vec<String>,
    pub stats: Vec<Stat>,
    pub links: HeroLinks,
    pub description: Vec<TextSegment>,
}

#[derive(Deserialize, Clone)]
pub struct HeroButtons {
    pub primary: String,
    pub secondary: String,
    pub tertiary: String,
    pub dot: String,
}

#[derive(Deserialize, Clone)]
pub struct Stat {
    pub value: String,
    pub label: String,
}

#[derive(Deserialize, Clone)]
pub struct HeroLinks {
    #[serde(default)]
    pub projects: String,
    #[serde(default)]
    pub resume: String,
    #[serde(default)]
    pub contact: String,
    #[serde(default)]
    pub github: Option<String>,
}

#[derive(Deserialize, Clone)]
#[serde(tag = "type")]
pub enum TextSegment {
    #[serde(rename = "plain")]
    Plain { text: String },
    #[serde(rename = "highlight")]
    Highlight { text: String, color: String },
}
