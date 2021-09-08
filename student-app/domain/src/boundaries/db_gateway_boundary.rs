use crate::boundaries::shared_boundary::SortDirectionRequest;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[async_trait]
pub trait StudentDbGateway {
    async fn find_one_by_id(&self, id: Uuid) -> Option<StudentDbResponse>;
    async fn exists_by_id(&self, id: Uuid) -> bool;
    async fn insert(
        &mut self,
        db_request: StudentMutationDbRequest,
    ) -> Result<StudentDbResponse, DbError>;

    async fn update(
        &mut self,
        db_request: StudentMutationDbRequest,
    ) -> Result<StudentDbResponse, DbError>;

    async fn delete(&mut self, id: Uuid) -> Result<(), DbError>;

    async fn find_collection_by(
        &self,
        db_request: StudentQueryDbRequest,
    ) -> StudentCollectionDbResponse;
}

pub struct StudentQueryDbRequest {
    pub id: Option<Uuid>,
    pub sort_request: Option<StudentSortDbRequest>,
    pub offset: Option<i64>,
    pub count: Option<i64>,
}

pub struct StudentSortDbRequest {
    pub sort_criteria: Vec<StudentSortCriteriaDbRequest>,
}

pub struct StudentSortCriteriaDbRequest {
    pub field: StudentSortFieldDbRequest,
    pub direction: SortDirectionRequest,
}

pub enum StudentSortFieldDbRequest {
    FirstName,
    MiddleName,
    LastName,
}

pub struct StudentMutationDbRequest {
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

pub struct StudentDbResponse {
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

pub struct StudentCollectionDbResponse {
    pub collection: Vec<StudentDbResponse>,
    pub has_more: Option<bool>,
    pub total: i64,
}

#[derive(Debug)]
pub enum DbError {
    UniqueConstraintViolationError(String),
    UnknownError(String),
}
