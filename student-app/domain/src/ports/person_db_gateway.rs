use crate::entities::person::Person;
use crate::ports::insert_person_port::InsertPersonPort;
use async_trait::async_trait;
use chrono::NaiveDate;
use uuid::Uuid;

#[async_trait]
pub trait PersonDbGateway: InsertPersonPort {}

pub struct PersonMutationDbRequest {
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
    // pub educational_stages: Option<Vec<PersonUsecaseSharedEducationalStage>>,
    pub nationality: Option<String>,
    pub race: Option<String>,
    pub id_number_id: Option<Uuid>,
    pub id_number: Option<String>,
    pub id_number_provider: Option<String>,
    pub date_of_issue: Option<NaiveDate>,
    pub place_of_issue: Option<String>,
    pub address: Option<String>,
    // pub languages: Option<Vec<PersonUsecaseSharedLanguage>>,
}

pub struct PersonDbResponse {
    pub id: Uuid,
    pub polity_id: Option<uuid::Uuid>,
    pub saint_ids: Option<Vec<uuid::Uuid>>,
    pub title: Option<String>,
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub last_name: Option<String>,
    pub date_of_birth: Option<NaiveDate>,
    pub place_of_birth: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
}

impl Person {
    pub fn to_mutation_db_request(&self) -> PersonMutationDbRequest {
        let personal_id_number = self.personal_id_number.clone().unwrap();
        PersonMutationDbRequest {
            id: self.id,
            polity_id: self.polity_id,
            saint_ids: self.saint_ids.clone(),
            title: self.title.clone().map(|title| title.to_string()),
            first_name: self.first_name.clone(),
            middle_name: self.middle_name.clone(),
            last_name: self.last_name.clone(),
            date_of_birth: self.date_of_birth,
            place_of_birth: self.place_of_birth.clone(),
            email: self.email.clone(),
            phone: self.phone.clone(),
            nationality: self.nationality.clone(),
            race: self.race.clone(),
            id_number_id: personal_id_number.id,
            id_number: personal_id_number.id_number.clone(),
            id_number_provider: personal_id_number
                .code
                .clone()
                .map(|provider| provider.to_string()),
            date_of_issue: personal_id_number.date_of_issue,
            place_of_issue: personal_id_number.place_of_issue,
            address: self.address.clone(),
        }
    }
}
