use crate::openapi::ToOpenApi;
use domain::usecases::query_student_collection_usecase::QueryStudentCollectionUsecaseOutput;
use domain::usecases::QueryStudentUsecaseOutput;
use hvcg_academics_openapi_student::models::{Student, StudentView, StudentViewCollection};

impl ToOpenApi<StudentViewCollection> for QueryStudentCollectionUsecaseOutput {
    fn to_openapi(self) -> StudentViewCollection {
        // TODO Optimise
        let polity_name: Option<String> = Some("".to_string());
        // if self.polity.is_some() {
        //     let polity = self.polity.unwrap();
        //     polity_name = polity.name;
        // } else {
        //     polity_name = None;
        // }

        let students = self.collection;
        let student_views = students
            .into_iter()
            .map(|output| output.to_openapi())
            .collect::<Vec<StudentView>>()
            .to_vec();

        StudentViewCollection {
            students: student_views,
            has_more: None,
            total: None,
        }
    }
}

impl ToOpenApi<StudentView> for QueryStudentUsecaseOutput {
    fn to_openapi(self) -> StudentView {
        // TODO Optimise
        let polity_name: Option<String> = Some("".to_string());

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
