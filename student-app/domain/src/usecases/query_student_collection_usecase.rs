use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::ports::student_db_gateway::StudentDbGateway;
use crate::usecases::student_usecase_shared_models::QueryStudentUsecaseOutput;
use crate::SortDirection;
use async_trait::async_trait;

pub struct QueryStudentCollectionUsecaseInteractor<A: StudentDbGateway> {
    db_gateway: A,
}

#[async_trait]
pub trait QueryStudentCollectionUsecase {
    // InputBoundary
    async fn execute(
        &self,
        request: QueryStudentCollectionUsecaseInput,
    ) -> QueryStudentCollectionUsecaseOutput;
}

#[async_trait]
impl<A> QueryStudentCollectionUsecase for QueryStudentCollectionUsecaseInteractor<A>
where
    A: StudentDbGateway + Sync + Send,
{
    // async fn get_student(
    //     &self,
    //     request: QueryStudentCollectionUsecaseInput,
    // ) -> Option<QueryStudentCollectionUsecaseOutput> {
    //     todo!()
    // }

    async fn execute(
        &self,
        request: QueryStudentCollectionUsecaseInput,
    ) -> QueryStudentCollectionUsecaseOutput {
        todo!()
    }
}

impl<A> QueryStudentCollectionUsecaseInteractor<A>
where
    A: StudentDbGateway + Sync + Send,
{
    pub fn new(db_gateway: A) -> Self {
        QueryStudentCollectionUsecaseInteractor { db_gateway }
    }
}

pub struct QueryStudentCollectionUsecaseInput {
    pub id: Option<Uuid>,
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub undergraduate_school: Option<String>,
    pub date_of_birth: Option<DateTime<Utc>>,
    pub place_of_birth: Option<String>,
    pub polity_name: Option<String>,
    pub specialism: Option<String>,
    pub sort_request: Option<QueryStudentCollectionUsecaseInputSort>,
    pub offset: Option<i32>,
    pub count: Option<i32>,
}

pub struct QueryStudentCollectionUsecaseInputSort {
    pub sort_criteria: Vec<QueryStudentCollectionUsecaseInputSortCriteria>,
}

#[derive(PartialEq, Clone)]
pub struct QueryStudentCollectionUsecaseInputSortCriteria {
    pub field: QueryStudentCollectionUsecaseInputSortField,
    pub direction: SortDirection,
}

#[derive(PartialEq, Clone)]
pub enum QueryStudentCollectionUsecaseInputSortField {
    Name,
    ChristianName,
    PolityName,
    LocationName,
    PlaceOfBirth,
}

pub struct QueryStudentCollectionUsecaseOutput {
    pub collection: Vec<QueryStudentUsecaseOutput>, // I am cheating here
    pub has_more: Option<bool>,
    pub total: i64,
}
