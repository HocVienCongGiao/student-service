use uuid::Uuid;

pub(crate) struct EducationalStage {
    pub id: Option<Uuid>,
    pub educational_level: Option<EducationalLevel>,
    pub school_name: Option<String>,
    pub major: Option<String>,
    pub graduate_year: Option<i8>,
    pub person_id: Option<Uuid>,
}

impl EducationalStage {
    pub(crate) fn is_valid(&self) -> bool {
        true
    }
}

#[derive(PartialEq, Clone)]
#[repr(C)]
pub(crate) enum EducationalLevel {
    ElementarySchool,
    MiddleSchool,
    HighSchool,
    Bachelor,
    Master,
    Doctor,
    Other,
}

impl std::fmt::Display for EducationalLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            EducationalLevel::ElementarySchool => write!(f, "ELEMENTARYSCHOOL"),
            EducationalLevel::MiddleSchool => write!(f, "MIDDLESCHOOL"),
            EducationalLevel::HighSchool => write!(f, "HIGHSCHOOL"),
            EducationalLevel::Bachelor => write!(f, "BACHELOR"),
            EducationalLevel::Master => write!(f, "MASTER"),
            EducationalLevel::Doctor => write!(f, "DOCTOR"),
            EducationalLevel::Other => write!(f, "OTHER"),
        }
    }
}

impl std::str::FromStr for EducationalLevel {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "ELEMENTARYSCHOOL" => std::result::Result::Ok(EducationalLevel::ElementarySchool),
            "MIDDLESCHOOL" => std::result::Result::Ok(EducationalLevel::MiddleSchool),
            "HIGHSCHOOL" => std::result::Result::Ok(EducationalLevel::HighSchool),
            "BACHELOR" => std::result::Result::Ok(EducationalLevel::Bachelor),
            "MASTER" => std::result::Result::Ok(EducationalLevel::Master),
            "DOCTOR" => std::result::Result::Ok(EducationalLevel::Doctor),
            "OTHER" => std::result::Result::Ok(EducationalLevel::Other),
            _ => std::result::Result::Err(format!("Value not valid: {}", s)),
        }
    }
}
