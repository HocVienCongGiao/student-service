use crate::boundaries::shared_boundary::SortDirectionRequest;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[async_trait]
pub trait StudentQueryInteraction {
    // InputBoundary
    async fn get_student(&self, request: StudentQueryUsecaseInput) -> Option<StudentUsecaseOutput>;
    async fn get_students(
        &self,
        request: StudentQueryUsecaseInput,
    ) -> StudentCollectionUsecaseOutput;
}

#[async_trait]
pub trait StudentMutationInteraction {
    // InputBoundary
    async fn create_student(
        &mut self,
        request: StudentMutationUsecaseInput,
    ) -> Result<StudentUsecaseOutput, UsecaseError>;
    async fn update_student(
        &mut self,
        request: StudentMutationUsecaseInput,
    ) -> Result<StudentUsecaseOutput, UsecaseError>;
    async fn delete_student(
        &mut self,
        request: StudentMutationUsecaseInput,
    ) -> Result<(), UsecaseError>;
}

pub struct StudentQueryUsecaseInput {
    pub id: Option<Uuid>,
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub undergraduate_school: Option<String>,
    pub date_of_birth: Option<DateTime<Utc>>,
    pub place_of_birth: Option<String>,
    pub polity_name: Option<String>,
    pub specialism: Option<String>,
    pub sort_request: Option<StudentSortUsecaseInput>,
    pub offset: Option<i32>,
    pub count: Option<i32>,
}

pub struct StudentSortUsecaseInput {
    pub sort_criteria: Vec<StudentSortCriteriaUsecaseInput>,
}

#[derive(PartialEq, Clone)]
pub struct StudentSortCriteriaUsecaseInput {
    pub field: StudentSortFieldUsecaseInput,
    pub direction: SortDirectionRequest,
}

#[derive(PartialEq, Clone)]
pub enum StudentSortFieldUsecaseInput {
    Name,
    ChristianName,
    PolityName,
    LocationName,
    PlaceOfBirth,
}

pub struct StudentMutationUsecaseInput {
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

pub struct StudentCollectionUsecaseOutput {
    pub collection: Vec<StudentUsecaseOutput>,
    pub has_more: Option<bool>,
    pub total: i64,
}

pub struct StudentUsecaseOutput {
    pub id: Option<Uuid>,
    pub polity: Option<PolityUsecaseOutput>,
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

pub struct PolityUsecaseOutput {
    pub id: Uuid,
    pub name: Option<String>,
    pub location_name: Option<String>,
    pub location_address: Option<String>,
    pub location_email: Option<String>,
}

#[derive(Debug)]
pub enum UsecaseError {
    UniqueConstraintViolationError(String),
    IdCollisionError,
    InvalidInput,
    UnknownError(String),
    ResourceNotFound,
}
