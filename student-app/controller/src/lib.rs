use chrono::NaiveDate;
use domain::usecases::UsecaseError;
use hvcg_academics_openapi_student::models::{
    StudentSortCriteria, StudentUpsert as StudentUpsertOpenApi, StudentView as StudentViewOpenApi,
    StudentViewCollection,
};
use uuid::Uuid;
mod create_student;
mod get_one_student_by_id;
mod get_student_collection;
mod update_student;

pub mod openapi;

pub(crate) trait ToUsecaseInput<T> {
    fn to_usecase_input(self) -> T;
}

pub async fn get_one_student_by_id(id: Uuid) -> Option<StudentViewOpenApi> {
    get_one_student_by_id::from_uuid(id).await
}

pub async fn update_student(
    student_request: StudentUpsertOpenApi,
    id: Uuid,
) -> Result<StudentViewOpenApi, UsecaseError> {
    update_student::from_openapi(student_request, id).await
}

pub async fn create_student(
    student_request: StudentUpsertOpenApi,
) -> Result<StudentViewOpenApi, UsecaseError> {
    create_student::from_openapi(student_request).await
}

pub async fn get_students(query: StudentCollectionQuery) -> StudentViewCollection {
    get_student_collection::from_usecase_input(query.to_usecase_input()).await
}

pub struct StudentCollectionQuery {
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub undergraduate_school: Option<String>,
    pub date_of_birth: Option<NaiveDate>,
    pub place_of_birth: Option<String>,
    pub polity_name: Option<String>,
    //pub specialism: Option<String>,
    pub sorts: Option<Vec<StudentSortCriteria>>,
    pub offset: Option<i64>,
    pub count: Option<i64>,
}
