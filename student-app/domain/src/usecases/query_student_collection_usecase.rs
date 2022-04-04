use crate::ports::person::person_db_gateway::PersonDbGateway;
use async_trait::async_trait;
use chrono::NaiveDate;
use uuid::Uuid;

use crate::ports::student::models::student_dbresponse::StudentCollection as StudentCollectionDbResponse;
use crate::ports::student::student_db_gateway::StudentDbGateway;
use crate::usecases::student_usecase_shared_models::QueryStudentUsecaseOutput;
use crate::usecases::student_usecase_shared_models::{WithChristianName, WithPolity};
use crate::usecases::ToUsecaseOutput;
use crate::SortDirection;

pub struct QueryStudentCollectionUsecaseInteractor<A: StudentDbGateway, B: PersonDbGateway> {
    student_db_gateway: A,
    person_db_gateway: B,
}

impl<A, B> QueryStudentCollectionUsecaseInteractor<A, B>
where
    A: StudentDbGateway + Sync + Send,
    B: PersonDbGateway + Sync + Send,
{
    pub fn new(student_db_gateway: A, person_db_gateway: B) -> Self {
        QueryStudentCollectionUsecaseInteractor {
            student_db_gateway,
            person_db_gateway,
        }
    }
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
impl<A, B> QueryStudentCollectionUsecase for QueryStudentCollectionUsecaseInteractor<A, B>
where
    A: StudentDbGateway + Sync + Send,
    B: PersonDbGateway + Sync + Send,
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

        let students = collection_usecase_output.collection;

        QueryStudentCollectionUsecaseOutput {
            collection: students,
            has_more: collection_usecase_output.has_more,
            total: collection_usecase_output.total,
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
    pub collection: Vec<QueryStudentUsecaseOutput>,
    // I am cheating here
    pub has_more: Option<bool>,
    pub total: i64,
}

impl ToUsecaseOutput<QueryStudentCollectionUsecaseOutput> for StudentCollectionDbResponse {
    fn to_usecase_output(self) -> QueryStudentCollectionUsecaseOutput {
        let collection = self
            .collection
            .into_iter()
            .map(|student_db_response| student_db_response.to_usecase_output()) // fn in query_one_student_by_id_usecase
            .collect();
        QueryStudentCollectionUsecaseOutput {
            collection,
            has_more: self.has_more,
            total: self.total,
        }
    }
}
