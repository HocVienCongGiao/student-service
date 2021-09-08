use async_trait::async_trait;
use domain::boundaries::db_gateway_boundary::{
    DbError, StudentCollectionDbResponse, StudentDbGateway, StudentDbResponse,
    StudentMutationDbRequest, StudentQueryDbRequest,
};
use tokio_postgres::{Client, Error, Row};
use uuid::Uuid;

pub struct StudentRepository {
    pub client: Client,
}

#[async_trait]
impl StudentDbGateway for StudentRepository {
    async fn find_one_by_id(&self, id: Uuid) -> Option<StudentDbResponse> {
        todo!()
    }

    async fn exists_by_id(&self, id: Uuid) -> bool {
        todo!()
    }

    async fn insert(
        &mut self,
        db_request: StudentMutationDbRequest,
    ) -> Result<StudentDbResponse, DbError> {
        println!("Inserting to DB");
        Ok(StudentDbResponse {
            id: None,
            polity_id: None,
            saint_ids: None,
            title: None,
            first_name: None,
            middle_name: None,
            last_name: None,
            date_of_birth: None,
            place_of_birth: None,
            email: None,
            phone: None,
            undergraduate_school: None,
        })
    }

    async fn update(
        &mut self,
        db_request: StudentMutationDbRequest,
    ) -> Result<StudentDbResponse, DbError> {
        todo!()
    }

    async fn delete(&mut self, id: Uuid) -> Result<(), DbError> {
        todo!()
    }

    async fn find_collection_by(
        &self,
        db_request: StudentQueryDbRequest,
    ) -> StudentCollectionDbResponse {
        todo!()
    }
}
