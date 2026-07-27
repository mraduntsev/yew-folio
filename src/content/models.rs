pub struct Particle {
    pub x: f64,
    pub y: f64,
    pub size: f64,
    pub speed_x: f64,
    pub speed_y: f64,
}

pub const NAV_LINKS: &[(&str, &str)] = &[
    ("About", "#about"),
    ("Education", "#education"),
    ("Projects", "#projects"),
    ("Contact", "#contact"),
];

pub struct HeroData {
    pub name: &'static str,
    pub role: &'static str,
    pub description: &'static str,
    pub stats: Vec<(&'static str, &'static str)>,
    pub links: HeroLinks,
}

pub struct HeroLinks {
    pub projects: &'static str,
    pub resume: &'static str,
    pub contact: &'static str,
}

pub struct Skills {
    pub name: &'static str,
    pub icons: &'static str,
}

