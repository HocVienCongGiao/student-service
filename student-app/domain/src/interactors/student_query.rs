use crate::boundaries::db_gateway_boundary::StudentDbGateway;
use crate::boundaries::usecase_boundary::{
    StudentCollectionUsecaseOutput, StudentQueryInteraction, StudentQueryUsecaseInput,
    StudentUsecaseOutput,
};
use async_trait::async_trait;

pub struct StudentQueryInteractor<A: StudentDbGateway> {
    db_gateway: A,
}

#[async_trait]
impl<A> StudentQueryInteraction for StudentQueryInteractor<A>
where
    A: StudentDbGateway + Sync + Send,
{
    async fn get_student(&self, request: StudentQueryUsecaseInput) -> Option<StudentUsecaseOutput> {
        todo!()
    }

    async fn get_students(
        &self,
        request: StudentQueryUsecaseInput,
    ) -> StudentCollectionUsecaseOutput {
        todo!()
    }
}

impl<A> StudentQueryInteractor<A>
where
    A: StudentDbGateway + Sync + Send,
{
    pub fn new(db_gateway: A) -> Self {
        StudentQueryInteractor { db_gateway }
    }
}
