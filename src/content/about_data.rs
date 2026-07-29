pub struct Principle {
    pub icon: &'static str,
    pub title: &'static str,
    pub description: &'static str,
}

pub struct AboutHeader {
    pub number: &'static str,
    pub title: &'static str,
    pub description: Option<&'static str>,
    pub centered: bool,
}

pub struct Skill {
    pub gradient: &'static str,
    pub letter: &'static str,
    pub title: &'static str,
    pub technologies: &'static str,
}

pub const PRINCIPLES: &[Principle] = &[
    Principle {
        icon: "bolt",
        title: "Performance-first",
        description: "From microsecond-sensitive trading paths to batch data pipelines — I measure before I optimize.",
    },
    Principle {
        icon: "shield",
        title: "Reliability obsessed",
        description: "Type systems, property tests, and careful error handling. If it can fail, I want to know at compile time.",
    },
    Principle {
        icon: "terminal",
        title: "Clean APIs",
        description: "Interfaces should be obvious. I design libraries and services that are pleasant to use on day one.",
    },
];

pub const ABOUT_SECTION: AboutHeader = AboutHeader {
    number: "01 — about",
    title: "A bit about me",
    description: {Some("I care about code that's boring in production — predictable, observable, and easy to delete.")},
    centered: false
};

pub const SKILLS: &[Skill] = &[
    Skill {
        gradient: "from-orange-600 to-red-700",
        letter: "R",
        title: "Rust",
        technologies: "tokio · axum · yew",
    },
    Skill {
        gradient: "from-blue-600 to-blue-800",
        letter: "C++",
        title: "C++",
        technologies: "17/20 · cmake · qt",
    },
    Skill {
        gradient: "from-yellow-400 to-blue-500",
        letter: "Py",
        title: "Python",
        technologies: "fastapi · django · pyqt/pyside · ml",
    },
    Skill {
        gradient: "from-sky-500 to-indigo-700",
        letter: "Pg",
        title: "PostgreSQL",
        technologies: "query tuning · migrations",
    },
    Skill {
        gradient: "from-cyan-400 to-blue-600",
        letter: "D",
        title: "Docker · K8s",
        technologies: "compose · helm",
    },
    Skill {
        gradient: "from-ink-700 to-ink-900",
        letter: "🐧",
        title: "Linux",
        technologies: "nix · ubuntu · debian",
    },
    Skill {
        gradient: "from-purple-500 to-pink-600",
        letter: "gR",
        title: "gRPC · Protobuf",
        technologies: "tonic · prost",
    },
    Skill {
        gradient: "from-red-500 to-red-700",
        letter: "Rd",
        title: "Redis",
        technologies: "streams · pub/sub",
    },
];
