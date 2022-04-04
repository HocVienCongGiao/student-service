use crate::openapi::ToOpenApi;
use db_postgres::person_gateway::repository::PersonRepository;
use db_postgres::student_gateway::repository::StudentRepository;
use domain::usecases::query_one_student_by_id_usecase::{
    QueryOneStudentByIdUsecase, QueryOneStudentByIdUsecaseInteractor,
};
use hvcg_academics_openapi_student::models::StudentView;
use uuid::Uuid;

pub(crate) async fn from_uuid(id: Uuid) -> Option<StudentView> {
    // Init dependencies
    let client = db_postgres::connect().await;
    let student_repository = StudentRepository { client };

    let person_client = db_postgres::connect().await;
    let person_repository = PersonRepository {
        client: person_client,
    };

    // Inject dependencies to Interactor and invoke func
    let query_one_student_usecase_output =
        QueryOneStudentByIdUsecaseInteractor::new(student_repository, person_repository)
            .execute(id)
            .await;

    query_one_student_usecase_output
        .map(|query_one_student_usecase_output| query_one_student_usecase_output.to_openapi())
}
