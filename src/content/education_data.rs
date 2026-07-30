pub struct EducationEntry {
    pub university: &'static str,
    pub degree: &'static str,
    pub years: &'static str,
    pub description: &'static str,
    pub highlights: Vec<&'static str>,
    pub badge: Option<&'static str>,
}

pub fn get_education_entries() -> Vec<EducationEntry> {
    vec![
        EducationEntry {
            university: "Chelyabinsk State University (CSU)",
            degree: "B.Sc. in Applied Mathematics and Computer Science",
            years: "2016 — 2021",
            description:
                "Rigorous program with a deep mathematical foundation and exceptionally high selection standards. \
         Graduated with excellent academic standing, focusing on computational systems, \
         algorithm optimization, and data structures.",
            badge: Some("B.Sc."),
            highlights: vec![
                "GPA: 4.4 / 5.0 — Excellent academic standing",
                "Advanced Mathematical Modeling",
                "Algorithms & Data Structures",
                "Computational Systems",
            ],
        },
        EducationEntry {
            university: "Ural State University of Railway Transport (USURT)",
            degree: "Postgraduate Researcher in Civil Engineering (Degree not conferred)",
            years: "2012 — 2015",
            description:
                "Successfully completed the full doctoral-level research cycle and coursework in structural \
         engineering. Independently designed, bootstrapped, and executed industrial prototyping \
         and mechanical testing at a manufacturing plant. Note: The degree was not formally awarded \
         due to administrative restructuring of the department, despite the research and patent \
         being successfully finalized.",
            badge: Some("Research"),
            highlights: vec![
                "Co‑inventor of patented structural technology (Patent RU 2535761)",
                "Managed end-to-end R&D cycle: design → manufacturing → physical testing",
                "Applied mechanical analysis and mathematical modeling to validate hypotheses",
            ],
        },
        EducationEntry {
            university: "Ural State University of Railway Transport (USURT)",
            degree: "Specialist Degree (M.Sc. equivalent) in Civil Engineering",
            years: "2000 — 2005",
            description:
                "Comprehensive 5-year engineering curriculum covering structural mechanics, \
                 strength of materials, and construction management. Developed a foundational \
                 mindset for complex system analysis.",
            badge: Some("Specialist"),
            highlights: vec![
                "Industrial and Civil Construction specialization",
                "Structural Mechanics & Material Science",
                "Engineering Project Management",
            ],
        },
    ]
}

pub struct Course {
    pub year: &'static str,
    pub hours: &'static str,
    pub title: &'static str,
    pub provider: &'static str,
    pub description: &'static str,
    pub tags: Vec<&'static str>,
}

pub fn get_courses() -> Vec<Course> {
    vec![
        Course {
            year: "2025",
            hours: "3 months",
            title: "DWH developer",
            provider: "T1 Digital Academy",
            description: "Data Warehousing (DWH) Architecture, SQL, ETL/ELT processes, Data Orchestration (Apache Airflow), Greenplum, Arenadata DB.",
            tags: vec!["dwh", "greenplum", "airflow"],
        },
        Course {
            year: "2023",
            hours: "200h",
            title: "Machine Learning and Data Analysis",
            provider: "Coursera / Stepik · Moscow Institute of Physics and Technology (MIPT) & Yandex",
            description: "Comprehensive specialization covering the full Machine Learning pipeline. \
                  Includes mathematical foundations (linear algebra, probability), \
                  data preprocessing, classical ML algorithms (Decision Trees, SVM, Clustering, \
                  Ensemble Methods), Neural Networks (Keras/TensorFlow), and applied data analysis \
                  using Python, Pandas, and Scikit-learn.",
            tags: vec!["Machine Learning", "Python", "Data Analysis", "Scikit-learn", "Deep Learning"],
        },
        Course {
            year: "2022",
            hours: "24 months",
            title: "Data Science and Machine Learning",
            provider: "GB, Faculty of Artificial Intelligence",
            description: "Completed an extensive professional training program focused on the fundamentals of Data Science, \
         covering mathematical foundations for machine learning, statistical analysis, and Python-based data processing.",
            tags: vec!["PyTorch", "ML", "Computer Vision"],
        },
        Course {
            year: "2020",
            hours: "100h",
            title: "Deep Learning",
            provider: "Moscow Institute of Physics and Technology (MIPT) · Advanced training",
            description: "Neural networks, CNNs, RNNs, generative models. Implemented custom \
                          training loops in PyTorch for image recognition and time‑series \
                          forecasting.",
            tags: vec!["PyTorch", "ML", "Computer Vision"],
        },
        Course {
            year: "2019",
            hours: "18 months",
            title: "Advanced Software Engineering Program",
            provider: "GB, Python Faculty",
            description: "Comprehensive year-long intensive program focused on enterprise backend development, \
         system architecture, and database management. Developed and deployed multiple \
         production-ready web applications.",
            tags: vec!["Python", "Django", "PyQt", "Vue"],
        },
    ]
}