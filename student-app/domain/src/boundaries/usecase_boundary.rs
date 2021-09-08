use crate::boundaries::shared_boundary::SortDirectionRequest;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[async_trait]
pub trait StudentQueryInteraction {
    // InputBoundary
    async fn get_student(
        &self,
        request: StudentQueryUsecaseRequest,
    ) -> Option<StudentUsecaseResponse>;
    async fn get_students(
        &self,
        request: StudentQueryUsecaseRequest,
    ) -> StudentCollectionUsecaseResponse;
}

#[async_trait]
pub trait StudentMutationInteraction {
    // InputBoundary
    async fn create_student(
        &mut self,
        request: StudentMutationUsecaseRequest,
    ) -> Result<StudentUsecaseResponse, UsecaseError>;
    async fn update_student(
        &mut self,
        request: StudentMutationUsecaseRequest,
    ) -> Result<StudentUsecaseResponse, UsecaseError>;
    async fn delete_student(
        &mut self,
        request: StudentMutationUsecaseRequest,
    ) -> Result<(), UsecaseError>;
}

pub struct StudentQueryUsecaseRequest {
    pub id: Option<Uuid>,
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub undergraduate_school: Option<String>,
    pub date_of_birth: Option<DateTime<Utc>>,
    pub place_of_birth: Option<String>,
    pub polity_name: Option<String>,
    pub specialism: Option<String>,
    pub sort_request: Option<StudentSortUsecaseRequest>,
    pub offset: Option<i32>,
    pub count: Option<i32>,
}

pub struct StudentSortUsecaseRequest {
    pub sort_criteria: Vec<StudentSortCriteriaUsecaseRequest>,
}

#[derive(PartialEq, Clone)]
pub struct StudentSortCriteriaUsecaseRequest {
    pub field: StudentSortFieldUsecaseRequest,
    pub direction: SortDirectionRequest,
}

#[derive(PartialEq, Clone)]
pub enum StudentSortFieldUsecaseRequest {
    Name,
    ChristianName,
    PolityName,
    LocationName,
    PlaceOfBirth,
}

pub struct StudentMutationUsecaseRequest {
    pub id: Option<Uuid>,
    pub polity_id: Option<Uuid>,
    pub saint_ids: Option<Vec<uuid::Uuid>>,
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

pub struct StudentCollectionUsecaseResponse {
    pub collection: Vec<StudentUsecaseResponse>,
    pub has_more: Option<bool>,
    pub total: i64,
}

pub struct StudentUsecaseResponse {
    pub id: Option<Uuid>,
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

#[derive(Debug)]
pub enum UsecaseError {
    UniqueConstraintViolationError(String),
    IdCollisionError,
    InvalidInput,
    UnknownError(String),
    ResourceNotFound,
}
