use crate::openapi::ToOpenApi;
use domain::usecases::create_student_usecase::CreateStudentUsecaseOutput;
use domain::usecases::student_usecase_shared_models::StudentUsecaseSharedTitle;
use hvcg_academics_openapi_student::models::{StudentTitle as StudentTitleOpenApi, StudentUpsert};
use uuid::Uuid;

impl ToOpenApi<StudentTitleOpenApi> for StudentUsecaseSharedTitle {
    fn to_openapi(self) -> StudentTitleOpenApi {
        match self {
            StudentUsecaseSharedTitle::Monk => StudentTitleOpenApi::MONK,
            StudentUsecaseSharedTitle::Nun => StudentTitleOpenApi::NUN,
            StudentUsecaseSharedTitle::Priest => StudentTitleOpenApi::PRIEST,
        }
    }
}
