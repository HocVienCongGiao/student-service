use crate::openapi::ToOpenApi;
use domain::boundaries::usecase_boundary::StudentUsecaseResponse;
use hvcg_academics_openapi_student::models::Student;

impl ToOpenApi<Student> for StudentUsecaseResponse {
    fn to_openapi(self) -> Student {
        Student {
            id: self.id,
            polity_id: self.polity_id,
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
