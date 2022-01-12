use chrono::{DateTime, NaiveDate, Utc};
use uuid::Uuid;

use crate::ports::polity_db_gateway::PolityDbGateway;
use crate::ports::saint_db_gateway::SaintDbGateway;
use crate::ports::student_db_gateway::{StudentCollectionDbResponse, StudentDbGateway};
use crate::usecases::student_usecase_shared_models::QueryStudentUsecaseOutput;
use crate::usecases::student_usecase_shared_models::{WithChristianName, WithPolity};
use crate::usecases::ToUsecaseOutput;
use crate::SortDirection;
use async_trait::async_trait;

pub struct QueryStudentCollectionUsecaseInteractor<
    A: StudentDbGateway,
    B: PolityDbGateway,
    C: SaintDbGateway,
> {
    student_db_gateway: A,
    polity_db_gateway: B,
    saint_db_gateway: C,
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
impl<A, B, C> QueryStudentCollectionUsecase for QueryStudentCollectionUsecaseInteractor<A, B, C>
where
    A: StudentDbGateway + Sync + Send,
    B: PolityDbGateway + Sync + Send,
    C: SaintDbGateway + Sync + Send,
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
        let collection_usecase_output = (*self)
            .student_db_gateway
            .find_collection_by(request.to_query_db_request())
            .await
            .to_usecase_output();

        let collection = collection_usecase_output.collection;
        let mut students: Vec<QueryStudentUsecaseOutput> = Vec::new();
        for e in collection {
            let mut student: QueryStudentUsecaseOutput = e;
            student = student.with_polity(
                Some("1".to_string()),
                Some("1".to_string()),
                Some("1".to_string()),
                Some("1".to_string()),
            );
            if let Some(polity_id) = student.polity_id {
                if let Some(polity_db_response) =
                    (*self).polity_db_gateway.find_one_by_id(polity_id).await
                {
                    student = student.with_polity(
                        polity_db_response.name,
                        polity_db_response.location_name,
                        polity_db_response.location_address,
                        polity_db_response.location_email,
                    )
                }
            }
            let saint_ids = student.saint_ids.clone();
            if let Some(saint_ids) = saint_ids {
                for (_i, e) in saint_ids.iter().enumerate() {
                    if let Some(saint_db_response) =
                        (*self).saint_db_gateway.find_one_by_id(*e).await
                    {
                        student = student.with_christian_name(saint_db_response.display_name)
                    }
                }
            }
            students.push(student);
        }
        QueryStudentCollectionUsecaseOutput {
            collection: students,
            has_more: collection_usecase_output.has_more,
            total: collection_usecase_output.total,
        }
    }
}

impl<A, B, C> QueryStudentCollectionUsecaseInteractor<A, B, C>
where
    A: StudentDbGateway + Sync + Send,
    B: PolityDbGateway + Sync + Send,
    C: SaintDbGateway + Sync + Send,
{
    pub fn new(student_db_gateway: A, polity_db_gateway: B, saint_db_gateway: C) -> Self {
        QueryStudentCollectionUsecaseInteractor {
            student_db_gateway,
            polity_db_gateway,
            saint_db_gateway,
        }
    }
}

pub struct QueryStudentCollectionUsecaseInput {
    pub id: Option<Uuid>,
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub undergraduate_school: Option<String>,
    pub date_of_birth: Option<NaiveDate>,
    pub place_of_birth: Option<String>,
    pub polity_name: Option<String>,
    //pub specialism: Option<String>,
    pub sort_request: Option<QueryStudentCollectionUsecaseInputSort>,
    pub offset: Option<i64>,
    pub count: Option<i64>,
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
    FirstName,
    MiddleName,
    LastName,
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

impl ToUsecaseOutput<QueryStudentCollectionUsecaseOutput> for StudentCollectionDbResponse {
    fn to_usecase_output(self) -> QueryStudentCollectionUsecaseOutput {
        // let collection = self
        //     .collection
        //     .into_iter()
        //     .map(|student_db_response| student_db_response.to_usecase_output()) // fn in query_one_student_by_id_usecase
        //     .collect();
        let collection = vec![]; // todo()!
        QueryStudentCollectionUsecaseOutput {
            collection,
            has_more: self.has_more,
            total: self.total,
        }
    }
}
