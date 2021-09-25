use crate::openapi::ToOpenApi;
use chrono::{DateTime, Utc};
use db_postgres::student_gateway::repository::StudentRepository;
use domain::boundaries::db_gateway_boundary::{StudentDbGateway, StudentQueryDbRequest};
use domain::boundaries::usecase_boundary::{
    StudentMutationInteraction, StudentMutationUsecaseInput, StudentQueryUsecaseInput, UsecaseError,
};
use domain::interactors::student_mutation::StudentMutationInteractor;
use hvcg_academics_openapi_student::models::{
    Student as StudentOpenApi, StudentSortCriteria, StudentViewCollection,
};

pub(crate) async fn from_usecase_request(
    request: StudentQueryUsecaseInput,
) -> StudentViewCollection {
    // Init dependencies
    let client = db_postgres::connect().await;
    let student_repository = StudentRepository { client };
    student_repository.find_collection_by(StudentQueryDbRequest {
        id: None,
        name: None,
        email: None,
        phone: None,
        undergraduate_school: None,
        date_of_birth: None,
        place_of_birth: None,
        polity_name: None,
        specialism: None,
        sort_request: None,
        offset: None,
        count: None,
    });
    StudentViewCollection {
        students: None,
        has_more: None,
        total: None,
    }
}
