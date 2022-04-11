use crate::ports::student::student_db_gateway::StudentDbGateway;
use crate::usecases::UsecaseError;
use async_trait::async_trait;
use uuid::Uuid;

pub struct DeleteOneStudentByIdUsecaseInteractor<A: StudentDbGateway> {
    student_db_gateway: A,
}

impl<A> DeleteOneStudentByIdUsecaseInteractor<A>
where
    A: StudentDbGateway + Sync + Send,
{
    pub fn new(student_db_gateway: A) -> Self {
        DeleteOneStudentByIdUsecaseInteractor { student_db_gateway }
    }
}

#[async_trait]
pub trait DeleteStudentUsecase {
    async fn execute(&mut self, id: Uuid) -> Result<(), UsecaseError>;
}

#[async_trait]
impl<A> DeleteStudentUsecase for DeleteOneStudentByIdUsecaseInteractor<A>
where
    A: StudentDbGateway + Sync + Send,
{
    async fn execute(&mut self, id: Uuid) -> Result<(), UsecaseError> {
        let result = (*self).student_db_gateway.delete(id).await;
        match result {
            Err(e) => Err(e.to_usecase_error()),
            _ => Ok(()),
        }
    }
}
