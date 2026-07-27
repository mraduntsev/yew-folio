use crate::content::{HeroData, HeroLinks};

pub fn get_hero_data() -> HeroData {
    HeroData {
        name: "Maxim",
        role: "Backend engineer",
        description: "I design low-latency services, high-performance APIs and reliable infrastructure that scales from prototype to production.",
        stats: vec![
            ("6+", "years"),
            ("40+", "projects"),
            ("12", "open source"),
        ],
        links: HeroLinks {
            projects: "#projects",
            resume: "/resume.pdf",
            contact: "#contact",
        },
    }
}