use crate::content::{SectionDescription, SectionHeader};

pub struct ContactChannel {
    pub label: &'static str,
    pub value: &'static str,
    pub icon: &'static str,
    pub url: Option<&'static str>,
    pub description: &'static str,
}

pub const CONTACTS_SECTION: SectionHeader = SectionHeader {
    number: "04 — contact",
    title: "Selected work",
    description: Some(SectionDescription::Text ("Let's build something")),
    centered: true,
};

pub fn get_contact_channels() -> Vec<ContactChannel> {
    vec![
        ContactChannel {
            label: "email",
            value: "raduntsev.mv@gmail.com",
            icon: "mail",
            url: Some("mailto:raduntsev.mv@gmail.com"),
            description: "Best for project inquiries",
        },
        ContactChannel {
            label: "telegram",
            value: "@reliable_it",
            icon: "telegram",
            url: Some("https://t.me/reliable_it"),
            description: "Quick replies, usually",
        },
        ContactChannel {
            label: "github",
            value: "@mraduntsev",
            icon: "github",
            url: Some("https://github.com/mraduntsev"),
            description: "Code, issues, PRs",
        },
        ContactChannel {
            label: "linkedin",
            value: "/in/maxim-raduntsev",
            icon: "linkedin",
            url: Some("https://linkedin.com/in/maxim-raduntsev"),
            description: "Professional network",
        },
    ]
}