use chrono::{DateTime, Utc};
use domain::usecases::UsecaseError;
use hvcg_academics_openapi_student::models::{
    Student as StudentOpenApi, StudentSortCriteria, StudentViewCollection,
};
use uuid::Uuid;

mod create_student;
mod get_students;
pub mod openapi;

pub async fn get_student_by_id(id: Uuid) -> Option<StudentOpenApi> {
    todo!()
}

pub async fn update_student(student_request: Option<StudentOpenApi>) -> Option<StudentOpenApi> {
    todo!()
}

pub async fn create_student(
    student_request: &StudentOpenApi,
) -> Result<StudentOpenApi, UsecaseError> {
    create_student::from_openapi(student_request).await
}

pub async fn get_students(query: StudentCollectionQuery) -> StudentViewCollection {
    get_students::from_usecase_input(query.to_usecase_input()).await
}

pub struct StudentCollectionQuery {
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub undergraduate_school: Option<String>,
    pub date_of_birth: Option<DateTime<Utc>>,
    pub place_of_birth: Option<String>,
    pub polity_name: Option<String>,
    pub specialism: Option<String>,
    pub sorts: Option<Vec<StudentSortCriteria>>,
    pub offset: Option<i32>,
    pub count: Option<i32>,
}
