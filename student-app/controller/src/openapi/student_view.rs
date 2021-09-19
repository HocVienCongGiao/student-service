use crate::openapi::ToOpenApi;
use domain::boundaries::usecase_boundary::StudentUsecaseResponse;
use hvcg_academics_openapi_student::models::{Student, StudentView};
use uuid::Uuid;

impl ToOpenApi<StudentView> for StudentUsecaseResponse {
    fn to_openapi(self) -> StudentView {
        // TODO Optimise
        let polity_name: Option<String>;
        if self.polity.is_some() {
            let polity = self.polity.unwrap();
            polity_name = polity.name;
        } else {
            polity_name = None;
        }

        StudentView {
            id: self.id,
            polity_name,
            polity_location_name: None,
            polity_location_address: None,
            polity_location_email: None,
            title: self.title,
            date_of_birth: self.date_of_birth,
            place_of_birth: self.place_of_birth,
            email: self.email,
            phone: self.phone,
            undergraduate_school: self.undergraduate_school,
            christian_name: None,
            name: Some(format!(
                "{} {} {}",
                self.last_name.clone().unwrap(),
                self.middle_name.clone().unwrap(),
                self.last_name.unwrap(),
            )),
            specialism: None,
        }
    }
}
