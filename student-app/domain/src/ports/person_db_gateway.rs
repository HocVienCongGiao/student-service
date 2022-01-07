use crate::entities::person::Person;
use crate::entities::person_educational_stage::EducationalStage;
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
    pub vow_progress: Option<String>,
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub last_name: Option<String>,
    pub date_of_birth: Option<NaiveDate>,
    pub place_of_birth: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub nationality: Option<String>,
    pub race: Option<String>,
    pub date_of_issue: Option<NaiveDate>,
    pub place_of_issue: Option<String>,
    pub address: Option<String>,
}

pub struct PersonDbResponse {
    pub id: Option<Uuid>,
    pub polity_id: Option<Uuid>,
    pub saint_ids: Option<Vec<uuid::Uuid>>,
    pub title: Option<String>,
    pub vow_progress: Option<String>,
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub last_name: Option<String>,
    pub date_of_birth: Option<NaiveDate>,
    pub place_of_birth: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub nationality: Option<String>,
    pub race: Option<String>,
    pub date_of_issue: Option<NaiveDate>,
    pub place_of_issue: Option<String>,
    pub address: Option<String>,
}

pub struct EducationalStageMutationDbRequest {
    pub educational_stage_list: Option<Vec<EducationalStage>>,
}

pub struct EducationalStageDbResponse {
    pub id: Option<Uuid>,
    pub educational_level: Option<EducationalLevel>,
    pub school_name: Option<String>,
    pub major: Option<String>,
    pub graduate_year: Option<i8>,
    pub person_id: Option<Uuid>,
}

impl Person {
    pub fn to_mutation_db_request(&self) -> PersonMutationDbRequest {
        PersonMutationDbRequest {
            id: self.id,
            polity_id: self.polity_id,
            saint_ids: self.saint_ids.clone(),
            title: self.title.clone().map(|title| title.to_string()),
            vow_progress: self
                .vow_progress
                .clone()
                .map(|vow_progress| vow_progress.to_string()),
            first_name: self.first_name.clone(),
            middle_name: self.middle_name.clone(),
            last_name: self.last_name.clone(),
            date_of_birth: self.date_of_birth,
            place_of_birth: self.place_of_birth.clone(),
            email: self.email.clone(),
            phone: self.phone.clone(),
            nationality: self.nationality,
            race: self.race,
            date_of_issue: self.date_of_issue,
            place_of_issue: self.place_of_issue,
            address: self.address,
        }
    }
}
