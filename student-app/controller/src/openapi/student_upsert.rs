use crate::openapi::ToOpenApi;
use domain::usecases::create_student_usecase::CreateStudentUsecaseOutput;
use domain::usecases::student_usecase_shared_models::StudentUsecaseSharedTitle;
use hvcg_academics_openapi_student::models::{StudentTitle as StudentTitleOpenApi, StudentUpsert};
use uuid::Uuid;

impl ToOpenApi<StudentUpsert> for CreateStudentUsecaseOutput {
    fn to_openapi(self) -> StudentUpsert {
        // TODO Optimise
        let polity_id: Option<Uuid> = self.polity_id;
        // if self.polity.is_some() {
        //     polity_id = Some(self.polity.unwrap().id);
        // } else {
        //     polity_id = None;
        // }

        StudentUpsert {
            polity_id,
            saint_id_array: self.saint_ids,
            title: self.title.map(|t| t.to_openapi()),
            first_name: self.first_name,
            middle_name: self.middle_name,
            last_name: self.last_name,
            date_of_birth: self.date_of_birth,
            place_of_birth: self.place_of_birth,
            email: self.email,
            phone: self.phone,
            undergraduate_school: self.undergraduate_school,
        }
    }
}

impl ToOpenApi<StudentTitleOpenApi> for StudentUsecaseSharedTitle {
    fn to_openapi(self) -> StudentTitleOpenApi {
        match self {
            StudentUsecaseSharedTitle::Monk => StudentTitleOpenApi::MONK,
            StudentUsecaseSharedTitle::Nun => StudentTitleOpenApi::NUN,
            StudentUsecaseSharedTitle::Priest => StudentTitleOpenApi::PRIEST,
        }
    }
}
