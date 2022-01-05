use uuid::Uuid;

pub(crate) struct ForeignLanguage {
    pub id: Option<Uuid>,
    pub person_id: Option<Uuid>,
    pub language: Option<String>,
    pub level: Option<ForeignLanguageLevel>,
}

impl ForeignLanguage {
    pub(crate) fn is_valid(&self) -> bool {
        true
    }
}

#[derive(PartialEq, Clone)]
#[repr(C)]
pub(crate) enum ForeignLanguageLevel {
    Beginner,
    Intermidiate,
    Advanced,
}

impl std::fmt::Display for ForeignLanguageLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ForeignLanguageLevel::Beginner => write!(f, "BEGINNER"),
            ForeignLanguageLevel::Intermidiate => write!(f, "INTERMIDIATE"),
            ForeignLanguageLevel::Advanced => write!(f, "ADVANCED"),
        }
    }
}

impl std::str::FromStr for ForeignLanguageLevel {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "BEGINNER" => std::result::Result::Ok(ForeignLanguageLevel::Beginner),
            "INTERMIDIATE" => std::result::Result::Ok(ForeignLanguageLevel::Intermidiate),
            "ADVANCED" => std::result::Result::Ok(ForeignLanguageLevel::Advanced),
            _ => std::result::Result::Err(format!("Value not valid: {}", s)),
        }
    }
}
