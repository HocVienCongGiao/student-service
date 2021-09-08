use chrono::{DateTime, Utc};
use domain::boundaries::usecase_boundary::UsecaseError;
use hvcg_academics_openapi_student::models::{
    Student as StudentOpenApi, StudentSortCriteria, StudentViewCollection,
};
use uuid::Uuid;

pub mod openapi;

mod converter;
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
    StudentViewCollection {
        students: None,
        has_more: None,
        total: None,
    }
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
