use crate::student_gateway::repository::StudentRepository;
use async_trait::async_trait;
use chrono::DateTime;
use domain::ports::find_student_collection_port::FindStudentCollectionPort;
use domain::ports::student_db_gateway::{
    StudentCollectionDbResponse, StudentDbResponse, StudentQueryDbRequest,
};
use std::str::FromStr;
use uuid::Uuid;

#[async_trait]
impl FindStudentCollectionPort for StudentRepository {
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
                // polity: Option::from(PolityDbResponse {
                //     id: Default::default(),
                //     name: Some("Empty".to_string()),
                //     location_name: None,
                //     location_address: None,
                //     location_email: None,
                // }),
                polity_id: None,
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
                polity_id: None,
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
