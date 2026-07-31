use crate::content::{SectionDescription, SectionHeader};

#[derive(PartialEq)]
pub struct EducationEntry {
    pub university: &'static str,
    pub degree: &'static str,
    pub years: &'static str,
    pub description: &'static str,
    pub highlights: &'static [&'static str],
    pub badge: &'static str,
}

pub struct Course {
    pub year: &'static str,
    pub hours: &'static str,
    pub title: &'static str,
    pub provider: &'static str,
    pub description: &'static str,
    pub tags: &'static [&'static str],
}

pub const EDUCATION_SECTION: SectionHeader = SectionHeader {
    number: "02 — education",
    title: "Learning path",
    description: Some(SectionDescription::Text("Formal CS background plus a steady diet of deep-dive courses and certifications.")),
    centered: false
};

pub const EDUCATION_ENTRIES: &[EducationEntry] = &[
    EducationEntry {
        university: "Chelyabinsk State University (CSU)",
        degree: "B.Sc. in Applied Mathematics and Computer Science",
        years: "2016 - 2021",
        description: "Rigorous program with a deep mathematical foundation and high selection standards. Graduated with strong academic standing, focusing on computational systems,   algorithm optimization, and data structures.",
        badge: "B.Sc.",
        highlights: &[
            "GPA: 4.36 / 5.0 — Very Good academic standing",
            "Advanced Mathematical Modeling",
            "Algorithms & Data Structures",
            "Computational Systems",
        ],
    },
    EducationEntry {
        university: "Ural State University of Railway Transport (USURT)",
        degree: "Postgraduate Researcher in Civil Engineering (Degree not conferred)",
        years: "2012 - 2015",
        description: "Successfully completed the full doctoral-level research cycle and coursework in structural engineering. Independently designed, bootstrapped, and executed industrial prototyping and mechanical testing at a manufacturing plant. \
         Note: The degree was not formally awarded due to administrative restructuring of the department, despite the research and patent being successfully finalized.",
        badge: "Research",
        highlights: &[
            "Co‑inventor of patented structural technology (Patent RU 2535761)",
            "Managed end-to-end R&D cycle: design → manufacturing → physical testing",
            "Applied mechanical analysis and mathematical modeling to validate hypotheses",
        ],
    },
    EducationEntry {
        university: "Ural State University of Railway Transport (USURT)",
        degree: "Specialist Degree (M.Sc. equivalent) in Civil Engineering",
        years: "2000 - 2005",
        description: "Comprehensive 5-year engineering curriculum covering structural mechanics.",
        badge: "Specialist",
        highlights: &[
            "Industrial and Civil Construction specialization",
            "Structural Mechanics & Material Science",
            "Engineering Project Management",
        ],
    },
];

pub const COURSES: &[Course] = &[
    Course {
        year: "2026",
        hours: "50h",
        title: "Applied Rust & WebAssembly Development",
        provider: "Self-directed study",
        description: "Mastered Rust's core: ownership, borrowing, lifetimes, and traits. Built a portfolio site on Yew/WASM to apply concepts in a real project with Cargo workspace.",
        tags: &["rust", "yew", "webassembly", "cargo", "async"],
    },
    Course {
        year: "2025",
        hours: "3 months",
        title: "DWH developer",
        provider: "T1 Digital Academy",
        description: "Designed and implemented MPP-based DWH architectures. Engineered ETL/ELT pipelines and data orchestration workflows using Apache Airflow. Deployed analytical workloads on Greenplum and Arenadata DB.",
        tags: &["dwh", "greenplum", "airflow", "etl"],
    },
    Course {
        year: "2023",
        hours: "200h",
        title: "Machine Learning and Data Analysis",
        provider: "MIPT & Yandex",
        description: "Rigorous specialization focusing on the mathematical foundations of ML (linear algebra, probability theory, optimization). Implemented classical algorithms (SVM, Ensemble Methods, Clustering) from scratch and via Scikit-learn, with a strong emphasis on statistical validation and data preprocessing.",
        tags: &["machine learning", "mathematics", "scikit-learn", "statistics"],
    },
    Course {
        year: "2022",
        hours: "24 months",
        title: "Data Science and Machine Learning",
        provider: "GB, Faculty of Artificial Intelligence",
        description: "Completed an extensive professional training program focused on the fundamentals of Data Science, \
         covering mathematical foundations for machine learning, statistical analysis, and Python-based data processing.",
        tags: &["pytorch", "mlops", "data engineering", "python"],
    },
    Course {
        year: "2020",
        hours: "100h",
        title: "Deep Learning",
        provider: "Moscow Institute of Physics and Technology (MIPT) · Advanced training",
        description: "Advanced study of neural network architectures (CNNs, RNNs, generative models). Implemented custom training loops and optimization strategies in PyTorch for complex image recognition and time-series forecasting tasks.",
        tags: &["pytorch", "deep learning", "computer vision", "time-series"],
    },
    Course {
        year: "2019",
        hours: "18 months",
        title: "Advanced Software Engineering Program",
        provider: "GB, Python Faculty",
        description: "Intensive program focused on enterprise backend development, distributed system architecture, and relational database management. Designed and deployed production-ready desktop and web applications using PyQt, Django, and Vue.",
        tags: &["python", "django", "system design", "pyqt", "vue"],
    },
];
