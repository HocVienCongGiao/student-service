pub mod create_student_usecase;
pub mod query_student_collection_usecase;
use chrono::{DateTime, Utc};
use uuid::Uuid;

pub(crate) trait ToEntity<T> {
    fn to_entity(self) -> T;
}

#[derive(Debug)]
pub enum UsecaseError {
    UniqueConstraintViolationError(String),
    IdCollisionError,
    InvalidInput,
    UnknownError(String),
    ResourceNotFound,
}

pub struct QueryStudentUsecaseOutput {
    pub id: Uuid,
    pub polity_id: Option<Uuid>,
    pub saint_ids: Option<Vec<Uuid>>,
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
