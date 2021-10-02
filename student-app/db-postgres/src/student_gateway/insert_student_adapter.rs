use crate::student_gateway::repository::StudentRepository;
use async_trait::async_trait;
use chrono::DateTime;
use domain::ports::insert_student_port::InsertStudentPort;
use domain::ports::student_db_gateway::{StudentDbResponse, StudentMutationDbRequest};
use domain::ports::DbError;
use uuid::Uuid;

#[async_trait]
impl InsertStudentPort for StudentRepository {
    async fn insert(
        &mut self,
        db_request: StudentMutationDbRequest,
    ) -> Result<StudentDbResponse, DbError> {
        println!("Inserting to DB");
        Ok(StudentDbResponse {
            id: Uuid::new_v4(),
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
}
