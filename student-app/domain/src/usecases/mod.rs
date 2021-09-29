pub mod create_student_usecase;
pub mod query_student_collection_usecase;
pub mod student_usecase_shared_models;

use chrono::{DateTime, Utc};
use uuid::Uuid;

pub(crate) trait ToEntity<T> {
    fn to_entity(self) -> T;
}

pub(crate) trait ToUsecaseOutput<T> {
    fn to_usecase_output(self) -> T;
}

#[derive(Debug)]
pub enum UsecaseError {
    UniqueConstraintViolationError(String),
    IdCollisionError,
    InvalidInput,
    UnknownError(String),
    ResourceNotFound,
}
