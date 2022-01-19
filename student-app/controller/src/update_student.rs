use crate::openapi::ToOpenApi;
use crate::ToUsecaseInput;
use db_postgres::polity_gateway::repository::PolityRepository;
use db_postgres::saint_gateway::repository::SaintRepository;
use db_postgres::student_gateway::repository::StudentRepository;
use domain::usecases::student_usecase_shared_models::StudentUsecaseSharedTitle;
use domain::usecases::update_student_usecase::{
    UpdateStudentUsecase, UpdateStudentUsecaseInput, UpdateStudentUsecaseInteractor,
};
use domain::usecases::UsecaseError;
use hvcg_academics_openapi_student::models::{
    StudentTitle, StudentUpsert as StudentUpsertOpenApi, StudentView as StudentViewOpenApi,
};
use uuid::Uuid;

pub(crate) async fn from_openapi(
    student: StudentUpsertOpenApi,
    id: Uuid,
) -> Result<StudentViewOpenApi, UsecaseError> {
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

    let usecase_input: UpdateStudentUsecaseInput = student.to_usecase_input();
    let usecase_input = usecase_input.with_id(id);

    // Inject dependencies to Interactor and invoke func
    let result = UpdateStudentUsecaseInteractor::new(
        student_repository,
        polity_repository,
        saint_repository,
    )
    .execute(usecase_input)
    .await;
    result.map(|res| res.to_openapi())
}

impl ToUsecaseInput<UpdateStudentUsecaseInput> for StudentUpsertOpenApi {
    fn to_usecase_input(self) -> UpdateStudentUsecaseInput {
        let mut title: Option<StudentUsecaseSharedTitle> = None;
        let title_openapi = self.title;
        if let Some(title_openapi) = title_openapi {
            title = Some(match title_openapi {
                StudentTitle::MONK => StudentUsecaseSharedTitle::Monk,
                StudentTitle::NUN => StudentUsecaseSharedTitle::Nun,
                StudentTitle::PRIEST => StudentUsecaseSharedTitle::Priest,
            })
        }

        UpdateStudentUsecaseInput {
            id: None,
            polity_id: self.polity_id,
            saint_ids: self.saint_id_array.clone(),
            title,
            first_name: self.first_name.clone(),
            middle_name: self.middle_name.clone(),
            last_name: self.last_name.clone(),
            date_of_birth: self.date_of_birth,
            place_of_birth: self.place_of_birth.clone(),
            email: self.email.clone(),
            phone: self.phone,
        }
    }
}
