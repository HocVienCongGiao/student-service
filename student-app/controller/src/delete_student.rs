use db_postgres::student_gateway::repository::StudentRepository;
use domain::usecases::delete_student_usecase::{
    DeleteStudentUsecase, DeleteStudentUsecaseInteractor,
};
use domain::usecases::UsecaseError;
use uuid::Uuid;

pub(crate) async fn from_uuid(id: Uuid) -> Result<(), UsecaseError> {
    // Init dependencies
    let client = db_postgres::connect().await;
    let student_repository = StudentRepository { client };

    // Inject dependencies to Interactor and invoke func
    let result = DeleteStudentUsecaseInteractor::new(student_repository)
        .execute(id)
        .await;
    result
}
