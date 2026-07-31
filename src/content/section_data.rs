use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub enum SectionDescription {
    Text(&'static str),
    Link {
        label: &'static str,
        href: &'static str,
    },
}

impl From<&'static str> for SectionDescription {
    fn from(s: &'static str) -> Self {
        SectionDescription::Text(s)
    }
}

impl From<(&'static str, &'static str)> for SectionDescription {
    fn from(tuple: (&'static str, &'static str)) -> Self {
        SectionDescription::Link {
            label: tuple.0,
            href: tuple.1,
        }
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct SectionHeader {
    pub number: &'static str,
    pub title: &'static str,
    pub description: Option<SectionDescription>,
    pub centered: bool,
}