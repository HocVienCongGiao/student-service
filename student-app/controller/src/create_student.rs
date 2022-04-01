use db_postgres::person_gateway::repository::PersonRepository;
use db_postgres::polity_gateway::repository::PolityRepository;
use db_postgres::saint_gateway::repository::SaintRepository;
use db_postgres::student_gateway::repository::StudentRepository;
use domain::usecases::create_student_usecase::{
    CreateStudentUsecase, CreateStudentUsecaseInput, CreateStudentUsecaseInteractor,
};
use domain::usecases::UsecaseError;
use hvcg_academics_openapi_student::models::StudentView as StudentViewOpenApi;


use crate::openapi::ToOpenApi;
use crate::{StudentUpsertOpenApi, ToUsecaseInput};

pub(crate) async fn from_openapi(
    student: StudentUpsertOpenApi,
) -> Result<StudentViewOpenApi, UsecaseError> {
    // Init dependencies
    let client = db_postgres::connect().await;
    let student_repository = StudentRepository { client };

    let person_client = db_postgres::connect().await;
    let person_repository = PersonRepository {
        client: person_client,
    };

    // Inject dependencies to Interactor and invoke func
    let result = CreateStudentUsecaseInteractor::new(student_repository, person_repository)
        .execute(student.to_usecase_input())
        .await;
    result.map(|res| res.to_openapi())
}

impl ToUsecaseInput<CreateStudentUsecaseInput> for StudentUpsertOpenApi {
    fn to_usecase_input(self) -> CreateStudentUsecaseInput {
        CreateStudentUsecaseInput {
            person_id: self.person_id,
        }
    }
}
