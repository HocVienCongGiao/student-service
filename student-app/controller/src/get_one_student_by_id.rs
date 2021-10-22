use crate::openapi::ToOpenApi;
use db_postgres::polity_gateway::repository::PolityRepository;
use db_postgres::saint_gateway::repository::SaintRepository;
use db_postgres::student_gateway::repository::StudentRepository;
use domain::usecases::query_one_student_by_id_usecase::{
    QueryOneStudentByIdUsecase, QueryOneStudentByIdUsecaseInteractor,
};
use domain::usecases::query_student_collection_usecase::{
    QueryStudentCollectionUsecaseInputSortField, QueryStudentCollectionUsecaseInteractor,
};
use hvcg_academics_openapi_student::models::StudentView;
use uuid::Uuid;

pub(crate) async fn from_uuid(id: Uuid) -> Option<StudentView> {
    // Init dependencies
    let client = db_postgres::connect().await;
    let student_repository = StudentRepository { client };

    let polity_client = db_postgres::connect().await;
    let polity_repository = PolityRepository {
        client: polity_client,
    };

    let saint_client = db_postgres::connect().await;
    let saint_repository = SaintRepository {
        client: saint_client,
    };

    // Inject dependencies to Interactor and invoke func
    let query_one_student_usecase_output = QueryOneStudentByIdUsecaseInteractor::new(
        student_repository,
        polity_repository,
        saint_repository,
    )
    .execute(id)
    .await;

    query_one_student_usecase_output
        .map(|query_one_student_usecase_output| query_one_student_usecase_output.to_openapi())
}
