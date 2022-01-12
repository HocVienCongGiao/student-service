use crate::entities::person::Person;

pub(crate) struct PersonLanguage {
    pub person: Person,
    pub language: Option<String>,
    pub level: Option<LanguageLevel>,
}

impl PersonLanguage {
    pub(crate) fn is_valid(&self) -> bool {
        true
    }
}

#[derive(PartialEq, Clone)]
#[repr(C)]
pub(crate) enum LanguageLevel {
    Beginner,
    Intermediate,
    Advanced,
}

impl std::fmt::Display for LanguageLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            LanguageLevel::Beginner => write!(f, "BEGINNER"),
            LanguageLevel::Intermediate => write!(f, "INTERMEDIATE"),
            LanguageLevel::Advanced => write!(f, "ADVANCED"),
        }
    }
}

impl std::str::FromStr for LanguageLevel {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "BEGINNER" => std::result::Result::Ok(LanguageLevel::Beginner),
            "INTERMEDIATE" => std::result::Result::Ok(LanguageLevel::Intermediate),
            "ADVANCED" => std::result::Result::Ok(LanguageLevel::Advanced),
            _ => std::result::Result::Err(format!("Value not valid: {}", s)),
        }
    }
}
