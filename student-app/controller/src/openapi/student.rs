use crate::openapi::ToOpenApi;
use domain::usecases::create_student_usecase::CreateStudentUsecaseOutput;
use hvcg_academics_openapi_student::models::{Student, StudentView};
use uuid::Uuid;

impl ToOpenApi<Student> for CreateStudentUsecaseOutput {
    fn to_openapi(self) -> Student {
        // TODO Optimise
        let polity_id: Option<Uuid> = self.polity_id;
        // if self.polity.is_some() {
        //     polity_id = Some(self.polity.unwrap().id);
        // } else {
        //     polity_id = None;
        // }

        Student {
            id: self.id,
            polity_id,
            saint_id_array: self.saint_ids,
            title: self.title,
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
