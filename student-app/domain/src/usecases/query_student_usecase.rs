use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::usecases::student_db_gateway::StudentDbGateway;
use crate::SortDirection;
use async_trait::async_trait;

pub struct QueryStudentUsecaseInteractor<A: StudentDbGateway> {
    db_gateway: A,
}

#[async_trait]
pub trait QueryStudentUsecase {
    // InputBoundary
    async fn get_student(
        &self,
        request: QueryStudentUsecaseInput,
    ) -> Option<QueryStudentUsecaseOutput>;
    async fn get_students(
        &self,
        request: QueryStudentUsecaseInput,
    ) -> QueryStudentCollectionUsecaseOutput;
}

#[async_trait]
impl<A> QueryStudentUsecase for QueryStudentUsecaseInteractor<A>
where
    A: StudentDbGateway + Sync + Send,
{
    async fn get_student(
        &self,
        request: QueryStudentUsecaseInput,
    ) -> Option<QueryStudentUsecaseOutput> {
        todo!()
    }

    async fn get_students(
        &self,
        request: QueryStudentUsecaseInput,
    ) -> QueryStudentCollectionUsecaseOutput {
        todo!()
    }
}

impl<A> QueryStudentUsecaseInteractor<A>
where
    A: StudentDbGateway + Sync + Send,
{
    pub fn new(db_gateway: A) -> Self {
        QueryStudentUsecaseInteractor { db_gateway }
    }
}

pub struct QueryStudentUsecaseInput {
    pub id: Option<Uuid>,
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub undergraduate_school: Option<String>,
    pub date_of_birth: Option<DateTime<Utc>>,
    pub place_of_birth: Option<String>,
    pub polity_name: Option<String>,
    pub specialism: Option<String>,
    pub sort_request: Option<QueryStudentUsecaseInputSort>,
    pub offset: Option<i32>,
    pub count: Option<i32>,
}

pub struct QueryStudentUsecaseInputSort {
    pub sort_criteria: Vec<QueryStudentUsecaseInputSortCriteria>,
}

#[derive(PartialEq, Clone)]
pub struct QueryStudentUsecaseInputSortCriteria {
    pub field: QueryStudentUsecaseInputSortField,
    pub direction: SortDirection,
}

#[derive(PartialEq, Clone)]
pub enum QueryStudentUsecaseInputSortField {
    Name,
    ChristianName,
    PolityName,
    LocationName,
    PlaceOfBirth,
}

pub struct QueryStudentUsecaseOutput {
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

pub struct QueryStudentCollectionUsecaseOutput {
    pub collection: Vec<QueryStudentUsecaseOutput>,
    pub has_more: Option<bool>,
    pub total: i64,
}
