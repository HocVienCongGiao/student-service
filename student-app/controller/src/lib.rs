use crate::get_students::StudentCollectionQuery;
use chrono::{DateTime, Utc};
use domain::usecases::UsecaseError;
use hvcg_academics_openapi_student::models::{
    Student as StudentOpenApi, StudentSortCriteria, StudentViewCollection,
};
use uuid::Uuid;

pub mod openapi;

mod create_student;
mod get_students;

pub async fn get_student_by_id(id: Uuid) -> Option<StudentOpenApi> {
    None
}

pub async fn update_student(student_request: Option<StudentOpenApi>) -> Option<StudentOpenApi> {
    None
}

pub async fn create_student(
    student_request: &StudentOpenApi,
) -> Result<StudentOpenApi, UsecaseError> {
    create_student::from_openapi(student_request).await
}

pub async fn get_students(query: StudentCollectionQuery) -> StudentViewCollection {
    get_students::from_usecase_input(query.to_usecase_input()).await
}
