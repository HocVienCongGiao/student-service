use crate::openapi::ToOpenApi;
use domain::usecases::create_student_usecase::CreateStudentUsecaseOutput;
use domain::usecases::query_student_collection_usecase::QueryStudentCollectionUsecaseOutput;
use domain::usecases::student_usecase_shared_models::QueryStudentUsecaseOutput;
use hvcg_academics_openapi_student::models::{StudentView, StudentViewCollection};

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
            person_id: None,
            student_id: None,
        }
    }
}

impl ToOpenApi<StudentView> for CreateStudentUsecaseOutput {
    fn to_openapi(self) -> StudentView {
        StudentView {
            person_id: Some(self.person_id),
            student_id: Some(self.student_id),
        }
    }
}
