pub struct SocialLink {
    pub label: &'static str,
    pub url: &'static str,
    pub icon: Option<&'static str>,
}

pub const SOCIAL_LINKS: &[SocialLink] = &[
    SocialLink {
        label: "Resume",
        url: "assets/resume.pdf",
        icon: Some("download"),
    },
    SocialLink {
        label: "GitHub",
        url: "https://github.com/mraduntsev",
        icon: None,
    },
    SocialLink {
        label: "Telegram",
        url: "https://t.me/reliable_it",
        icon: None,
    },
    SocialLink {
        label: "Email",
        url: "mailto:raduntsev.mv@gmail.com",
        icon: None,
    },
    SocialLink {
        label: "top",
        url: "#hero",
        icon: Some("arrow-up"),
    },
];

pub const COPYRIGHT: &str = "© 2026 Raduntsev Maxim. Built with Rust + Yew.";