use crate::openapi::ToOpenApi;
use domain::usecases::create_student_usecase::CreateStudentUsecaseOutput;
use domain::usecases::query_student_collection_usecase::QueryStudentCollectionUsecaseOutput;
use domain::usecases::student_usecase_shared_models::QueryStudentUsecaseOutput;
use hvcg_academics_openapi_student::models::{StudentView, StudentViewCollection};
use uuid::Uuid;

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
            has_more: self.has_more,
            total: Some(self.total),
        }
    }
}

impl ToOpenApi<StudentView> for QueryStudentUsecaseOutput {
    fn to_openapi(self) -> StudentView {
        StudentView {
            id: self.id,
            polity_name: self.polity_name,
            polity_location_name: self.polity_location_name,
            polity_location_address: self.polity_location_address,
            polity_location_email: self.polity_location_email,
            title: self.title.map(|t| t.parse().unwrap()),
            date_of_birth: self.date_of_birth,
            place_of_birth: self.place_of_birth,
            email: self.email,
            phone: self.phone,
            undergraduate_school: self.undergraduate_school,
            christian_name: self.christian_name.map(|saint_names| saint_names.join(" ")),
            name: Some(format!(
                "{}_{}_{}",
                self.first_name.unwrap(),
                self.middle_name.unwrap(),
                self.last_name.unwrap(),
            )),
        }
    }
}

impl ToOpenApi<StudentView> for CreateStudentUsecaseOutput {
    fn to_openapi(self) -> StudentView {
        // TODO Optimise
        let polity_id: Option<Uuid> = self.polity_id;
        // if self.polity.is_some() {
        //     polity_id = Some(self.polity.unwrap().id);
        // } else {
        //     polity_id = None;
        // }

        StudentView {
            id: self.id,
            polity_name: self.polity_name,
            polity_location_name: self.polity_location_name,
            polity_location_address: self.polity_location_address,
            polity_location_email: self.polity_location_email,
            title: self.title.map(|t| t.to_openapi()),
            date_of_birth: self.date_of_birth,
            place_of_birth: self.place_of_birth,
            email: self.email,
            phone: self.phone,
            undergraduate_school: self.undergraduate_school,
            christian_name: self.christian_name.map(|saint_names| saint_names.join(" ")),
            name: Some(format!(
                "{} {} {}",
                self.last_name.unwrap_or_default(),
                self.middle_name.unwrap_or_default(),
                self.first_name.unwrap_or_default(),
            )),
        }
    }
}
