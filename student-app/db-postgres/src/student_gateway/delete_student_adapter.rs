use crate::student_gateway::repository::StudentRepository;
use async_trait::async_trait;
use domain::ports::delete_student_port::DeleteStudentPort;
use domain::ports::DbError;
use tokio_postgres::types::ToSql;
use uuid::Uuid;

#[async_trait]
impl DeleteStudentPort for StudentRepository {
    async fn delete(&mut self, id: Uuid) -> Result<(), DbError> {
        let stmt = self
            .client
            .prepare("DELETE FROM public.student__student WHERE id = $1")
            .await
            .unwrap();

        let name_params: &[&(dyn ToSql + Sync)] = &[&id];
        let result = self.client.execute(&stmt, name_params).await;

        match result {
            Ok(_) => Ok(()),
            Err(error) => Err(DbError::UnknownError(
                error.into_source().unwrap().to_string(),
            )),
        }
    }
}
