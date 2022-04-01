use crate::entities::person::Person;
use crate::ports::person::insert_person_port::InsertPersonPort;
use crate::ports::person::models;
use crate::ports::person::models::person_mutation_dbrequest::PersonalIdNumber;
use async_trait::async_trait;
use models::person_mutation_dbrequest::Person as PersonMutationDbRequest;
use crate::ports::person::find_one_person_by_id_port::FindOnePersonByIdPort;

#[async_trait]
pub trait PersonDbGateway: InsertPersonPort + FindOnePersonByIdPort {}

impl Person {
    pub fn to_mutation_db_request(&self) -> PersonMutationDbRequest {
        let personal_id_number = self.personal_id_number.clone().unwrap();
        let personal_id_number_request = PersonalIdNumber {
            id: personal_id_number.id,
            id_number: personal_id_number.id_number,
            code: personal_id_number.code.clone().map(|code| code.to_string()),
            date_of_issue: personal_id_number.date_of_issue,
            place_of_issue: personal_id_number.place_of_issue,
        };
        PersonMutationDbRequest {
            id: self.person_id,
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
            personal_id_number: Some(personal_id_number_request),
            address: self.address.clone(),
        }
    }
}
