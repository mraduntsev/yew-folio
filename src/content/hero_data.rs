pub enum TextSegment {
    Plain(&'static str),
    Highlight { text: &'static str, color: &'static str },
}

pub struct HeroLinks {
    pub projects: &'static str,
    pub resume: &'static str,
    pub contact: &'static str,
    pub github: Option<&'static str>,
}

pub struct HeroData {
    pub name: &'static str,
    pub role: &'static str,
    pub description: Vec<TextSegment>,
    pub stats: Vec<(&'static str, &'static str)>,
    pub links: HeroLinks,
}

pub fn get_hero_data() -> HeroData {
    HeroData {
        name: "Maxim",
        role: "Backend engineer",
        description: vec![
            TextSegment::Plain("Backend engineer focused on ".into()),
            TextSegment::Highlight { text: "Rust".into(), color: "text-rust-500".into() },
            TextSegment::Plain(", ".into()),
            TextSegment::Highlight { text: "C++".into(), color: "".into() },
            TextSegment::Plain(" and ".into()),
            TextSegment::Highlight { text: "Python".into(), color: "".into() },
            TextSegment::Plain(". I design low-latency services, high-performance APIs and reliable infrastructure that scales from prototype to production.".into()),
        ],
        stats: vec![
            ("6+", "years"),
            ("40+", "projects"),
            ("12", "open source"),
        ],
        links: HeroLinks {
            projects: "#projects",
            resume: "assets/resume.pdf",
            contact: "#contact",
            github: None,
        },
    }
}