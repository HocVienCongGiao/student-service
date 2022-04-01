use async_trait::async_trait;
use tokio_postgres::types::ToSql;
use tokio_postgres::{Error, Transaction};
use uuid::Uuid;

use domain::entities::student::Student;
use domain::ports::student::insert_student_port::InsertStudentPort;
use domain::ports::DbError;

use crate::student_gateway::repository::StudentRepository;

#[async_trait]
impl InsertStudentPort for StudentRepository {
    async fn insert(&mut self, db_request: Student) -> Result<Student, DbError> {
        // insert student_id
        let person_id = db_request.person_id.unwrap();
        let student_id = db_request.student_id.unwrap();
        let stmt = (*self)
            .client
            .prepare("INSERT into public.student__student (id, person_id) VAlUES ($1, $2)")
            .await
            .unwrap();

        let params: &[&(dyn ToSql + Sync)] = &[&student_id, &person_id];
        let result = (*self).client.execute(&stmt, params).await;

        match result {
            Err(e) => Err(DbError::UnknownError(e.into_source().unwrap().to_string())),
            _ => Ok(Student {
                student_id: Some(student_id),
                person_id: Some(person_id),
            }),
        }
    }
}
