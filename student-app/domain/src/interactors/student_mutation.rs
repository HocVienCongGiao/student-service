use crate::boundaries::db_gateway_boundary::StudentDbGateway;
use crate::boundaries::usecase_boundary::{
    StudentMutationInteraction, StudentMutationUsecaseInput, StudentUsecaseOutput, UsecaseError,
};
use crate::interactors::ToEntity;
use async_trait::async_trait;
use uuid::Uuid;

pub struct StudentMutationInteractor<A: StudentDbGateway> {
    db_gateway: A,
}

#[async_trait]
impl<A> StudentMutationInteraction for StudentMutationInteractor<A>
where
    A: StudentDbGateway + Sync + Send,
{
    async fn create_student(
        &mut self,
        request: StudentMutationUsecaseInput,
    ) -> Result<StudentUsecaseOutput, UsecaseError> {
        let student = request.to_entity();
        if student.is_valid() {
            println!("This student is valid");
            (*self)
                .db_gateway
                .insert(student.to_mutation_db_request())
                .await
                .map(|response| response.to_usecase_response())
                .map_err(|err| err.to_usecase_error())
        } else {
            println!("This student is not valid");
            Err(UsecaseError::InvalidInput)
        }
    }

    async fn update_student(
        &mut self,
        request: StudentMutationUsecaseInput,
    ) -> Result<StudentUsecaseOutput, UsecaseError> {
        todo!()
    }

    async fn delete_student(
        &mut self,
        request: StudentMutationUsecaseInput,
    ) -> Result<(), UsecaseError> {
        todo!()
    }
}

impl<A> StudentMutationInteractor<A>
where
    A: StudentDbGateway + Sync + Send,
{
    pub fn new(db_gateway: A) -> Self {
        StudentMutationInteractor { db_gateway }
    }
}
