use crate::usecases::person_usecase_shared_models::{
    PersonUsecaseSharedEducationalStage, PersonUsecaseSharedIdNumberProvider,
    PersonUsecaseSharedLanguage,
};
use chrono::NaiveDate;
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

pub struct QueryStudentUsecaseOutput {
    pub person_id: Uuid,
    pub student_id: Option<Uuid>,
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
    pub date_of_birth: Option<NaiveDate>,
    pub place_of_birth: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub educational_stages: Option<Vec<PersonUsecaseSharedEducationalStage>>,
    pub nationality: Option<String>,
    pub race: Option<String>,
    pub id_number: Option<String>,
    pub id_number_provider: Option<PersonUsecaseSharedIdNumberProvider>,
    pub date_of_issue: Option<NaiveDate>,
    pub place_of_issue: Option<String>,
    pub address: Option<String>,
    pub foreign_language: Option<Vec<PersonUsecaseSharedLanguage>>,
}
