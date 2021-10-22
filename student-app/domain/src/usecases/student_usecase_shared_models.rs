use chrono::{DateTime, Utc};
use uuid::Uuid;

pub(crate) trait WithPolity<T> {
    fn with_polity(
        self,
        name: Option<String>,
        location_name: Option<String>,
        location_address: Option<String>,
        location_email: Option<String>,
    ) -> T;
}

pub(crate) trait WithChristianName<T> {
    fn with_christian_name(self, name: Option<String>) -> T;
}

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
    pub polity_name: Option<String>,
    pub polity_location_name: Option<String>,
    pub polity_location_address: Option<String>,
    pub polity_location_email: Option<String>,
    pub saint_ids: Option<Vec<Uuid>>,
    pub christian_name: Option<Vec<String>>,
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
