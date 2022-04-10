use db_postgres::student_gateway::repository::StudentRepository;
use domain::usecases::delete_one_student_by_id_usecase::{
    DeleteOneStudentByIdUsecaseInteractor, DeleteStudentUsecase,
};
use domain::usecases::UsecaseError;
use uuid::Uuid;

pub(crate) async fn from_uuid(id: Uuid) -> Result<(), UsecaseError> {
    // Init dependencies
    let client = db_postgres::connect().await;
    let repository = StudentRepository { client };

    let delete_one_person_usecase_output = DeleteOneStudentByIdUsecaseInteractor::new(repository)
        .execute(id)
        .await;
    delete_one_person_usecase_output
}
