use serde::Deserialize;
#[derive(Deserialize, Clone)]
pub struct NavLocale {
    pub buttons: NavButtons,
    pub nav: Vec<NavLinkData>,
}

#[derive(Deserialize, Clone)]
pub struct NavLinkData {
    pub label: String,
    pub href: String,
}

#[derive(Deserialize, Clone)]
pub struct NavButtons {
    pub primary: String,
    pub outline: String,
}
