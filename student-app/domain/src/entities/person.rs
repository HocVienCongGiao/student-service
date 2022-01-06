use chrono::NaiveDate;
use uuid::Uuid;

pub(crate) struct Person {
    pub id: Option<Uuid>,
    pub polity_id: Option<Uuid>,
    pub saint_ids: Option<Vec<Uuid>>,
    pub title: Option<PersonTitle>,
    pub vow_progress: Option<VowProgress>,
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub last_name: Option<String>,
    pub date_of_birth: Option<NaiveDate>,
    pub place_of_birth: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub nationality: Option<String>,
    pub race: Option<String>,
    pub address: Option<String>,
}

impl Person {
    pub(crate) fn is_valid(&self) -> bool {
        true
    }
}

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

#[derive(PartialEq, Clone)]
#[repr(C)]
pub(crate) enum VowProgress {
    SolemnVow,
    SimpleVow,
    Novoice,
    Preparation,
}

impl std::fmt::Display for VowProgress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            VowProgress::SolemnVow => write!(f, "SOLEMNVOW"),
            VowProgress::SimpleVow => write!(f, "SIMPLEVOW"),
            VowProgress::Novoice => write!(f, "NOVOICE"),
            VowProgress::Preparation => write!(f, "PREPARATION"),
        }
    }
}

impl std::str::FromStr for VowProgress {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "SOLEMNVOW" => std::result::Result::Ok(VowProgress::SolemnVow),
            "SIMPLEVOW" => std::result::Result::Ok(VowProgress::SimpleVow),
            "NOVOICE" => std::result::Result::Ok(VowProgress::Novoice),
            "PREPARATION" => std::result::Result::Ok(VowProgress::Preparation),
            _ => std::result::Result::Err(format!("Value not valid: {}", s)),
        }
    }
}
