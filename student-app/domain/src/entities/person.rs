use crate::entities::personal_id_number::PersonalIdNumber;
use chrono::NaiveDate;
use uuid::Uuid;

#[derive(Clone)]
pub(crate) struct Person {
    pub id: Option<Uuid>,
    pub polity_id: Option<Uuid>,
    pub saint_ids: Option<Vec<uuid::Uuid>>,
    pub title: Option<PersonTitle>,
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub last_name: Option<String>,
    pub date_of_birth: Option<NaiveDate>,
    pub place_of_birth: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    // pub educational_stages: Option<Vec<PersonUsecaseSharedEducationalStage>>,
    pub nationality: Option<String>,
    pub race: Option<String>,
    pub personal_id_number: Option<PersonalIdNumber>,
    pub address: Option<String>,
    // pub languages: Option<Vec<PersonUsecaseSharedLanguage>>,
}

// impl Person {
//     pub(crate) fn is_valid(&self) -> bool {
//         true
//     }
// }

#[derive(PartialEq, Clone)]
#[repr(C)]
pub(crate) enum PersonTitle {
    Priest,
    Monk,
    Nun,
}

impl std::fmt::Display for PersonTitle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            PersonTitle::Priest => write!(f, "PRIEST"),
            PersonTitle::Monk => write!(f, "MONK"),
            PersonTitle::Nun => write!(f, "NUN"),
        }
    }
}

impl std::str::FromStr for PersonTitle {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "PRIEST" => std::result::Result::Ok(PersonTitle::Priest),
            "MONK" => std::result::Result::Ok(PersonTitle::Monk),
            "NUN" => std::result::Result::Ok(PersonTitle::Nun),
            _ => std::result::Result::Err(format!("Value not valid: {}", s)),
        }
    }
}
