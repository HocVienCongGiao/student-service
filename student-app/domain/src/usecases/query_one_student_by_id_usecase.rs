use crate::ports::student_db_gateway::{StudentDbGateway, StudentDbResponse};
use crate::usecases::student_usecase_shared_models::QueryStudentUsecaseOutput;
use crate::usecases::ToUsecaseOutput;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct QueryOneStudentByIdUsecaseInteractor<A: StudentDbGateway> {
    db_gateway: A,
}

#[async_trait]
pub trait QueryOneStudentByIdUsecase {
    // InputBoundary
    async fn execute(&self, id: Uuid) -> Option<QueryStudentUsecaseOutput>;
}

#[async_trait]
impl<A> QueryOneStudentByIdUsecase for QueryOneStudentByIdUsecaseInteractor<A>
where
    A: StudentDbGateway + Sync + Send,
{
    async fn execute(&self, id: Uuid) -> Option<QueryStudentUsecaseOutput> {
        (*self)
            .db_gateway
            .find_one_by_id(id)
            .await
            .map(|response| response.to_usecase_output())
    }
}

impl<A> QueryOneStudentByIdUsecaseInteractor<A>
where
    A: StudentDbGateway + Sync + Send,
{
    pub fn new(db_gateway: A) -> Self {
        QueryOneStudentByIdUsecaseInteractor { db_gateway }
    }
}

impl ToUsecaseOutput<QueryStudentUsecaseOutput> for StudentDbResponse {
    fn to_usecase_output(self) -> QueryStudentUsecaseOutput {
        QueryStudentUsecaseOutput {
            id: self.id,
            polity_id: self.polity_id,
            saint_ids: self.saint_ids,
            title: self.title,
            first_name: self.first_name,
            middle_name: self.middle_name,
            last_name: self.last_name,
            date_of_birth: self.date_of_birth,
            place_of_birth: self.place_of_birth,
            email: self.email,
            phone: self.phone,
            undergraduate_school: self.undergraduate_school,
        }
    }
}
