#![allow(missing_docs, trivial_casts, unused_variables, unused_mut, unused_imports, unused_extern_crates, non_camel_case_types)]

use async_trait::async_trait;
use futures::Stream;
use std::error::Error;
use std::task::{Poll, Context};
use swagger::{ApiError, ContextWrapper};
use serde::{Serialize, Deserialize};

type ServiceError = Box<dyn Error + Send + Sync + 'static>;

pub const BASE_PATH: &'static str = "";
pub const API_VERSION: &'static str = "0.8.1";

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum AddStudentResponse {
    /// successful operation
    SuccessfulOperation
    (models::StudentUpsert)
    ,
    /// Invalid input
    InvalidInput
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum DeleteStudentResponse {
    /// Successful operation
    SuccessfulOperation
    ,
    /// Invalid ID supplied
    InvalidIDSupplied
    ,
    /// student not found
    StudentNotFound
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum UpdateStudentResponse {
    /// Successful operation
    SuccessfulOperation
    (models::StudentUpsert)
    ,
    /// Invalid ID supplied
    InvalidIDSupplied
    ,
    /// Student not found
    StudentNotFound
    ,
    /// Validation exception
    ValidationException
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum GetStudentByIdResponse {
    /// Successful operation
    SuccessfulOperation
    (models::StudentUpsert)
    ,
    /// Invalid ID supplied
    InvalidIDSupplied
    ,
    /// Student not found
    StudentNotFound
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum GetStudentsResponse {
    /// Successful operation
    SuccessfulOperation
    (models::StudentViewCollection)
    ,
    /// Invalid ID supplied
    InvalidIDSupplied
    ,
    /// Student not found
    StudentNotFound
}

/// API
#[async_trait]
pub trait Api<C: Send + Sync> {
    fn poll_ready(&self, _cx: &mut Context) -> Poll<Result<(), Box<dyn Error + Send + Sync + 'static>>> {
        Poll::Ready(Ok(()))
    }

    /// Add new student
    async fn add_student(
        &self,
        student_upsert: models::StudentUpsert,
        context: &C) -> Result<AddStudentResponse, ApiError>;

    /// Deletes a student
    async fn delete_student(
        &self,
        id: uuid::Uuid,
        context: &C) -> Result<DeleteStudentResponse, ApiError>;

    /// Update an existing student
    async fn update_student(
        &self,
        id: uuid::Uuid,
        student_upsert: models::StudentUpsert,
        context: &C) -> Result<UpdateStudentResponse, ApiError>;

    /// Find student by ID
    async fn get_student_by_id(
        &self,
        id: uuid::Uuid,
        context: &C) -> Result<GetStudentByIdResponse, ApiError>;

    /// Get all students
    async fn get_students(
        &self,
        name: Option<String>,
        email: Option<String>,
        phone: Option<String>,
        undergraduate_school: Option<String>,
        date_of_birth: Option<chrono::DateTime::<chrono::Utc>>,
        place_of_birth: Option<String>,
        polity_name: Option<String>,
        specialism: Option<String>,
        sorts: Option<&Vec<models::StudentSortCriteria>>,
        offset: Option<i32>,
        count: Option<i32>,
        context: &C) -> Result<GetStudentsResponse, ApiError>;

}

/// API where `Context` isn't passed on every API call
#[async_trait]
pub trait ApiNoContext<C: Send + Sync> {

    fn poll_ready(&self, _cx: &mut Context) -> Poll<Result<(), Box<dyn Error + Send + Sync + 'static>>>;

    fn context(&self) -> &C;

    /// Add new student
    async fn add_student(
        &self,
        student_upsert: models::StudentUpsert,
        ) -> Result<AddStudentResponse, ApiError>;

    /// Deletes a student
    async fn delete_student(
        &self,
        id: uuid::Uuid,
        ) -> Result<DeleteStudentResponse, ApiError>;

    /// Update an existing student
    async fn update_student(
        &self,
        id: uuid::Uuid,
        student_upsert: models::StudentUpsert,
        ) -> Result<UpdateStudentResponse, ApiError>;

    /// Find student by ID
    async fn get_student_by_id(
        &self,
        id: uuid::Uuid,
        ) -> Result<GetStudentByIdResponse, ApiError>;

    /// Get all students
    async fn get_students(
        &self,
        name: Option<String>,
        email: Option<String>,
        phone: Option<String>,
        undergraduate_school: Option<String>,
        date_of_birth: Option<chrono::DateTime::<chrono::Utc>>,
        place_of_birth: Option<String>,
        polity_name: Option<String>,
        specialism: Option<String>,
        sorts: Option<&Vec<models::StudentSortCriteria>>,
        offset: Option<i32>,
        count: Option<i32>,
        ) -> Result<GetStudentsResponse, ApiError>;

}

/// Trait to extend an API to make it easy to bind it to a context.
pub trait ContextWrapperExt<C: Send + Sync> where Self: Sized
{
    /// Binds this API to a context.
    fn with_context(self: Self, context: C) -> ContextWrapper<Self, C>;
}

impl<T: Api<C> + Send + Sync, C: Clone + Send + Sync> ContextWrapperExt<C> for T {
    fn with_context(self: T, context: C) -> ContextWrapper<T, C> {
         ContextWrapper::<T, C>::new(self, context)
    }
}

#[async_trait]
impl<T: Api<C> + Send + Sync, C: Clone + Send + Sync> ApiNoContext<C> for ContextWrapper<T, C> {
    fn poll_ready(&self, cx: &mut Context) -> Poll<Result<(), ServiceError>> {
        self.api().poll_ready(cx)
    }

    fn context(&self) -> &C {
        ContextWrapper::context(self)
    }

    /// Add new student
    async fn add_student(
        &self,
        student_upsert: models::StudentUpsert,
        ) -> Result<AddStudentResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().add_student(student_upsert, &context).await
    }

    /// Deletes a student
    async fn delete_student(
        &self,
        id: uuid::Uuid,
        ) -> Result<DeleteStudentResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().delete_student(id, &context).await
    }

    /// Update an existing student
    async fn update_student(
        &self,
        id: uuid::Uuid,
        student_upsert: models::StudentUpsert,
        ) -> Result<UpdateStudentResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().update_student(id, student_upsert, &context).await
    }

    /// Find student by ID
    async fn get_student_by_id(
        &self,
        id: uuid::Uuid,
        ) -> Result<GetStudentByIdResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_student_by_id(id, &context).await
    }

    /// Get all students
    async fn get_students(
        &self,
        name: Option<String>,
        email: Option<String>,
        phone: Option<String>,
        undergraduate_school: Option<String>,
        date_of_birth: Option<chrono::DateTime::<chrono::Utc>>,
        place_of_birth: Option<String>,
        polity_name: Option<String>,
        specialism: Option<String>,
        sorts: Option<&Vec<models::StudentSortCriteria>>,
        offset: Option<i32>,
        count: Option<i32>,
        ) -> Result<GetStudentsResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_students(name, email, phone, undergraduate_school, date_of_birth, place_of_birth, polity_name, specialism, sorts, offset, count, &context).await
    }

}


#[cfg(feature = "client")]
pub mod client;

// Re-export Client as a top-level name
#[cfg(feature = "client")]
pub use client::Client;

#[cfg(feature = "server")]
pub mod server;

// Re-export router() as a top-level name
#[cfg(feature = "server")]
pub use self::server::Service;

#[cfg(feature = "server")]
pub mod context;

pub mod models;

#[cfg(any(feature = "client", feature = "server"))]
pub(crate) mod header;
