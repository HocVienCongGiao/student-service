use uuid::Uuid;

pub(crate) struct IdNumber {
    pub id: Option<Uuid>,
    pub person_id: Option<Uuid>,
    pub id_number: Option<String>,
    pub id_number_provider: Option<PersonIdNumberProvider>,
    pub date_of_issue: Option<NaiveDate>,
    pub place_of_issue: Option<String>,
}

impl IdNumber {
    pub(crate) fn is_valid(&self) -> bool {
        true
    }
}

#[derive(PartialEq, Clone)]
#[repr(C)]
pub(crate) enum PersonIdNumberProvider {
    NationalId,
    Passport,
}

impl std::fmt::Display for PersonIdNumberProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            PersonIdNumberProvider::NationalId => write!(f, "NATIONAL_ID"),
            PersonIdNumberProvider::Passport => write!(f, "PASSPORT"),
        }
    }
}

impl std::str::FromStr for PersonIdNumberProvider {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "NATIONAL_ID" => std::result::Result::Ok(PersonIdNumberProvider::NationalId),
            "PASSPORT" => std::result::Result::Ok(PersonIdNumberProvider::Passport),
            _ => std::result::Result::Err(format!("Value not valid: {}", s)),
        }
    }
}
