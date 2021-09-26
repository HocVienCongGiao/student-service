pub mod create_student_usecase;
pub mod query_student_usecase;
pub mod student_db_gateway;

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
