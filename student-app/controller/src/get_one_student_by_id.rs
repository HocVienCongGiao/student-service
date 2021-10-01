use crate::openapi::ToOpenApi;
use crate::{StudentCollectionQuery, ToUsecaseInput};
use db_postgres::student_gateway::repository::StudentRepository;
use domain::usecases::query_one_student_by_id_usecase::{
    QueryOneStudentByIdUsecase, QueryOneStudentByIdUsecaseInteractor,
};
use domain::usecases::query_student_collection_usecase::{
    QueryStudentCollectionUsecase, QueryStudentCollectionUsecaseInput,
    QueryStudentCollectionUsecaseInputSort, QueryStudentCollectionUsecaseInputSortCriteria,
    QueryStudentCollectionUsecaseInputSortField, QueryStudentCollectionUsecaseInteractor,
};
use domain::usecases::student_usecase_shared_models::QueryStudentUsecaseOutput;
use domain::SortDirection;
use hvcg_academics_openapi_student::models::{StudentSortCriteria, StudentView};
use uuid::Uuid;

pub(crate) async fn from_uuid(id: Uuid) -> Option<StudentView> {
    // Init dependencies
    let client = db_postgres::connect().await;
    let student_repository = StudentRepository { client };

    // Inject dependencies to Interactor and invoke func
    let query_one_student_usecase_output =
        QueryOneStudentByIdUsecaseInteractor::new(student_repository)
            .execute(id)
            .await;

    let student_view_openapi = query_one_student_usecase_output
        .map(|query_one_student_usecase_output| query_one_student_usecase_output.to_openapi());

    student_view_openapi
}
