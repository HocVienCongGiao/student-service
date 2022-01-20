use crate::entities::person::Person;
use crate::entities::student::Student;
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
        }
    }
}
