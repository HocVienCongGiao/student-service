use chrono::NaiveDate;
use uuid::Uuid;

pub struct Person {
    pub id: Option<Uuid>,
    pub polity_id: Option<Uuid>,
    pub saint_ids: Option<Vec<uuid::Uuid>>,
    pub title: Option<String>,
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub last_name: Option<String>,
    pub date_of_birth: Option<NaiveDate>,
    pub place_of_birth: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub nationality: Option<String>,
    pub race: Option<String>,
    pub personal_id_number: Option<PersonalIdNumber>,
    pub address: Option<String>,
    // pub languages: Option<Vec<PersonUsecaseSharedLanguage>>,
    // pub educational_stages: Option<Vec<PersonUsecaseSharedEducationalStage>>,
}

pub struct PersonalIdNumber {
    pub id: Option<Uuid>,
    pub id_number: Option<String>,
    pub code: Option<String>,
    pub date_of_issue: Option<NaiveDate>,
    pub place_of_issue: Option<String>,
}
