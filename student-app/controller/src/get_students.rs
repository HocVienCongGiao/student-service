use crate::openapi::ToOpenApi;
use chrono::{DateTime, Utc};
use db_postgres::student_gateway::repository::StudentRepository;
use domain::boundaries::usecase_boundary::{
    StudentMutationInteraction, StudentMutationUsecaseRequest, StudentQueryUsecaseRequest,
    UsecaseError,
};
use domain::interactors::student_mutation::StudentMutationInteractor;
use hvcg_academics_openapi_student::models::{
    Student as StudentOpenApi, StudentSortCriteria, StudentViewCollection,
};

pub(crate) async fn from_usecase_request(
    request: StudentQueryUsecaseRequest,
) -> StudentViewCollection {
    // Init dependencies
    let client = db_postgres::connect().await;
    let student_repository = StudentRepository { client };

    StudentViewCollection {
        students: None,
        has_more: None,
        total: None,
    }
}
