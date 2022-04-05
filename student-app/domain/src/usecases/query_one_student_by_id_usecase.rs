use crate::entities::student::Student as StudentEntity;
use crate::ports::person::person_db_gateway::PersonDbGateway;
use async_trait::async_trait;
use uuid::Uuid;

use crate::ports::student::student_db_gateway::StudentDbGateway;
use crate::usecases::student_usecase_shared_models::{
    QueryStudentUsecaseOutput, WithChristianName, WithPolity,
};
use crate::usecases::ToUsecaseOutput;

pub struct QueryOneStudentByIdUsecaseInteractor<A: StudentDbGateway, B: PersonDbGateway> {
    student_db_gateway: A,
    person_db_gateway: B,
}

impl<A, B> QueryOneStudentByIdUsecaseInteractor<A, B>
where
    A: StudentDbGateway + Sync + Send,
    B: PersonDbGateway + Sync + Send,
{
    pub fn new(student_db_gateway: A, person_db_gateway: B) -> Self {
        QueryOneStudentByIdUsecaseInteractor {
            student_db_gateway,
            person_db_gateway,
        }
    }
}

#[async_trait]
pub trait QueryOneStudentByIdUsecase {
    // InputBoundary
    async fn execute(&self, id: Uuid) -> Option<QueryStudentUsecaseOutput>;
}

#[async_trait]
impl<A, B> QueryOneStudentByIdUsecase for QueryOneStudentByIdUsecaseInteractor<A, B>
where
    A: StudentDbGateway + Sync + Send,
    B: PersonDbGateway + Sync + Send,
{
    async fn execute(&self, id: Uuid) -> Option<QueryStudentUsecaseOutput> {
        let usecase_output: Option<QueryStudentUsecaseOutput> = (*self)
            .student_db_gateway
            .find_one_by_id(id)
            .await
            .map(|response| response.to_usecase_output());

        usecase_output
    }
}

impl ToUsecaseOutput<QueryStudentUsecaseOutput> for StudentEntity {
    fn to_usecase_output(self) -> QueryStudentUsecaseOutput {
        QueryStudentUsecaseOutput {
            student_id: self.student_id.unwrap(),
            person_id: self.person_id.unwrap(),
        }
    }
}
