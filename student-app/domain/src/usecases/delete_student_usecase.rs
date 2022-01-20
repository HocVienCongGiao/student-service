use async_trait::async_trait;
use uuid::Uuid;

use crate::ports::student_db_gateway::StudentDbGateway;
use crate::usecases::UsecaseError;

pub struct DeleteStudentUsecaseInteractor<A: StudentDbGateway> {
    student_db_gateway: A,
}

impl<A> DeleteStudentUsecaseInteractor<A>
where
    A: StudentDbGateway + Sync + Send,
{
    pub fn new(student_db_gateway: A) -> Self {
        DeleteStudentUsecaseInteractor { student_db_gateway }
    }
}

#[async_trait]
pub trait DeleteStudentUsecase {
    // InputBoundary
    async fn execute(&mut self, id: Uuid) -> Result<(), UsecaseError>;
}

#[async_trait]
impl<A> DeleteStudentUsecase for DeleteStudentUsecaseInteractor<A>
where
    A: StudentDbGateway + Sync + Send,
{
    async fn execute(&mut self, id: Uuid) -> Result<(), UsecaseError> {
        // id not exists
        let student_db_response = (*self).student_db_gateway.find_one_by_id(id).await;
        if student_db_response.is_none() {
            println!("student not exists");
            return Err(UsecaseError::ResourceNotFound);
        }

        // id exists
        (*self)
            .student_db_gateway
            .delete(id)
            .await
            .map_err(|err| err.to_usecase_error())
    }
}
