use async_trait::async_trait;
use chrono::DateTime;
use domain::boundaries::db_gateway_boundary::{
    DbError, PolityDbResponse, StudentCollectionDbResponse, StudentDbGateway, StudentDbResponse,
    StudentMutationDbRequest, StudentQueryDbRequest,
};
use std::str::FromStr;
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
            polity: None,
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
        let mut result = StudentCollectionDbResponse {
            collection: vec![],
            has_more: false,
            total: 3,
        };

        let mut students = vec![
            StudentDbResponse {
                id: Option::from(Uuid::from_str("53f549b9-99bf-4e12-88e3-c2f868953283").unwrap()),
                polity: Option::from(PolityDbResponse {
                    id: Default::default(),
                    name: Some("Empty".to_string()),
                    location_name: None,
                    location_address: None,
                    location_email: None,
                }),
                title: Some("PRIEST".to_string()),
                first_name: None,
                middle_name: None,
                date_of_birth: Option::from(
                    DateTime::from_str("1990-10-29 00:00:00+00:00").unwrap(),
                ),
                place_of_birth: Option::from("Tra Vinh".to_string()),
                email: Option::from("binh@sunrise.vn".to_string()),
                phone: Option::from("84 1228019700".to_string()),
                undergraduate_school: Option::from(
                    "Dai Chung Vien Thanh Quy - Can Tho".to_string(),
                ),
                saint_ids: None,
                last_name: None,
            },
            StudentDbResponse {
                id: Option::from(Uuid::from_str("53f549b9-99bf-4e12-88e3-c2f868953283").unwrap()),
                polity: None,
                title: Option::from("PRIEST".to_string()),
                first_name: None,
                middle_name: None,
                date_of_birth: Option::from(
                    DateTime::from_str("1990-10-29 00:00:00+00:00").unwrap(),
                ),
                place_of_birth: Option::from("Tra Vinh".to_string()),
                email: Option::from("binh@sunrise.vn".to_string()),
                phone: Option::from("84 1228019700".to_string()),
                undergraduate_school: Option::from(
                    "Dai Chung Vien Thanh Quy - Can Tho".to_string(),
                ),
                saint_ids: None,
                last_name: None,
            },
        ];
        result.collection.append(&mut students);
        result
    }
}
