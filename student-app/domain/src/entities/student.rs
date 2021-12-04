use chrono::{DateTime, Utc};
use uuid::Uuid;

pub(crate) struct Student {
    pub id: Option<Uuid>,
    pub polity_id: Option<Uuid>,
    pub saint_ids: Option<Vec<uuid::Uuid>>,
    pub title: Option<StudentTitle>,
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub last_name: Option<String>,
    pub date_of_birth: Option<DateTime<Utc>>,
    pub place_of_birth: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub undergraduate_school: Option<String>,
}

impl Student {
    pub(crate) fn is_valid(&self) -> bool {
        true
    }
}

#[derive(PartialEq, Clone)]
#[repr(C)]
pub(crate) enum StudentTitle {
    Priest,
    Monk,
    Nun,
}

impl std::fmt::Display for StudentTitle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            StudentTitle::Priest => write!(f, "{}", "PRIEST"),
            StudentTitle::Monk => write!(f, "{}", "MONK"),
            StudentTitle::Nun => write!(f, "{}", "NUN"),
        }
    }
}

impl std::str::FromStr for StudentTitle {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "PRIEST" => std::result::Result::Ok(StudentTitle::Priest),
            "MONK" => std::result::Result::Ok(StudentTitle::Monk),
            "NUN" => std::result::Result::Ok(StudentTitle::Nun),
            _ => std::result::Result::Err(format!("Value not valid: {}", s)),
        }
    }
}
