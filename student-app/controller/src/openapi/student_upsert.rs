use crate::openapi::ToOpenApi;
use domain::usecases::create_student_usecase::CreateStudentUsecaseOutput;
use domain::usecases::person_usecase_shared_models::PersonUsecaseSharedTitle;
use hvcg_academics_openapi_student::models::{StudentTitle as StudentTitleOpenApi, StudentUpsert};
use uuid::Uuid;

impl ToOpenApi<StudentTitleOpenApi> for PersonUsecaseSharedTitle {
    fn to_openapi(self) -> StudentTitleOpenApi {
        match self {
            PersonUsecaseSharedTitle::Monk => StudentTitleOpenApi::MONK,
            PersonUsecaseSharedTitle::Nun => StudentTitleOpenApi::NUN,
            PersonUsecaseSharedTitle::Priest => StudentTitleOpenApi::PRIEST,
        }
    }
}
