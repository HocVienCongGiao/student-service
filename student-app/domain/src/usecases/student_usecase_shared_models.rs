use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(PartialEq, Clone)]
#[allow(non_camel_case_types)]
#[repr(C)]
pub enum StudentUsecaseSharedTitle {
    Priest,
    Monk,
    Nun,
}

impl std::str::FromStr for StudentUsecaseSharedTitle {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "PRIEST" => std::result::Result::Ok(StudentUsecaseSharedTitle::Priest),
            "MONK" => std::result::Result::Ok(StudentUsecaseSharedTitle::Monk),
            "NUN" => std::result::Result::Ok(StudentUsecaseSharedTitle::Nun),
            _ => std::result::Result::Err(format!("Value not valid: {}", s)),
        }
    }
}

pub struct QueryStudentUsecaseOutput {
    pub id: Uuid,
    pub polity_id: Option<Uuid>,
    pub saint_ids: Option<Vec<Uuid>>,
    pub title: Option<String>,
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub last_name: Option<String>,
    pub date_of_birth: Option<DateTime<Utc>>,
    pub place_of_birth: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub undergraduate_school: Option<String>,
}
