use crate::usecases::UsecaseError;

pub mod delete_student_port;
pub mod find_one_student_by_id_port;
pub mod find_student_collection_port;
pub mod insert_student_port;
pub mod student_db_gateway;
pub mod update_student_port;
pub mod polity_db_gateway;
pub mod find_one_polity_by_id_port;

#[derive(Debug)]
pub enum DbError {
    UniqueConstraintViolationError(String),
    UnknownError(String),
}

impl DbError {
    pub(crate) fn to_usecase_error(&self) -> UsecaseError {
        match self {
            DbError::UniqueConstraintViolationError(field) => {
                UsecaseError::UniqueConstraintViolationError(field.to_string())
            }
            DbError::UnknownError(msg) => UsecaseError::UnknownError(msg.to_string()),
        }
    }
}