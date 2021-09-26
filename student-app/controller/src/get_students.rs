use crate::StudentCollectionQuery;
use chrono::{DateTime, Utc};
use domain::usecases::query_student_usecase::{
    QueryStudentUsecaseInput, QueryStudentUsecaseInputSort, QueryStudentUsecaseInputSortCriteria,
    QueryStudentUsecaseInputSortField, QueryStudentUsecaseOutput,
};
use domain::SortDirection;
use hvcg_academics_openapi_student::models::{
    Student as StudentOpenApi, StudentSortCriteria, StudentViewCollection,
};

pub(crate) async fn from_usecase_input(request: QueryStudentUsecaseInput) -> StudentViewCollection {
    // Init dependencies
    let client = db_postgres::connect().await;

    StudentViewCollection {
        students: None,
        has_more: None,
        total: None,
    }
}

impl StudentCollectionQuery {
    pub fn to_usecase_input(&self) -> QueryStudentUsecaseInput {
        QueryStudentUsecaseInput {
            id: None,
            name: self.name.clone(),
            email: self.email.clone(),
            phone: self.phone.clone(),
            undergraduate_school: self.undergraduate_school.clone(),
            date_of_birth: self.date_of_birth,
            place_of_birth: self.place_of_birth.clone(),
            polity_name: self.polity_name.clone(),
            specialism: self.specialism.clone(),
            sort_request: from_openapi_to_usecase_input(self.sorts.clone()),
            offset: self.offset,
            count: self.count,
        }
    }
}

pub fn from_openapi_to_usecase_input(
    openapi: Option<Vec<StudentSortCriteria>>,
) -> Option<QueryStudentUsecaseInputSort> {
    let sort_request: Option<QueryStudentUsecaseInputSort>;
    let sort_criteria_dto = openapi;

    if let Some(sort_criteria_dto) = sort_criteria_dto {
        let mut sort_criteria = Vec::new();
        sort_criteria_dto.iter().for_each(|criterion| {
            let sort_criteria_request = match criterion {
                StudentSortCriteria::NAME_ASC => build_sort_criteria_request(
                    QueryStudentUsecaseInputSortField::Name,
                    SortDirection::Asc,
                ),
                StudentSortCriteria::NAME_DESC => build_sort_criteria_request(
                    QueryStudentUsecaseInputSortField::Name,
                    SortDirection::Desc,
                ),
                StudentSortCriteria::CHRISTIAN_NAME_ASC => build_sort_criteria_request(
                    QueryStudentUsecaseInputSortField::ChristianName,
                    SortDirection::Asc,
                ),
                StudentSortCriteria::CHRISTIAN_NAME_DESC => build_sort_criteria_request(
                    QueryStudentUsecaseInputSortField::ChristianName,
                    SortDirection::Desc,
                ),
                StudentSortCriteria::POLITY_NAME_ASC => build_sort_criteria_request(
                    QueryStudentUsecaseInputSortField::PolityName,
                    SortDirection::Asc,
                ),
                StudentSortCriteria::POLITY_NAME_DESC => build_sort_criteria_request(
                    QueryStudentUsecaseInputSortField::PolityName,
                    SortDirection::Desc,
                ),
                StudentSortCriteria::LOCATION_NAME_ASC => build_sort_criteria_request(
                    QueryStudentUsecaseInputSortField::LocationName,
                    SortDirection::Asc,
                ),
                StudentSortCriteria::LOCATION_NAME_DESC => build_sort_criteria_request(
                    QueryStudentUsecaseInputSortField::LocationName,
                    SortDirection::Desc,
                ),
                StudentSortCriteria::PLACE_OF_BIRTH_ASC => build_sort_criteria_request(
                    QueryStudentUsecaseInputSortField::PlaceOfBirth,
                    SortDirection::Asc,
                ),
                StudentSortCriteria::PLACE_OF_BIRTH_DESC => build_sort_criteria_request(
                    QueryStudentUsecaseInputSortField::PlaceOfBirth,
                    SortDirection::Desc,
                ),
            };
            sort_criteria.push(sort_criteria_request);
        });

        sort_request = Option::from(QueryStudentUsecaseInputSort { sort_criteria });
    } else {
        sort_request = None;
    }
    sort_request
}

fn build_sort_criteria_request(
    field: QueryStudentUsecaseInputSortField,
    direction: SortDirection,
) -> QueryStudentUsecaseInputSortCriteria {
    QueryStudentUsecaseInputSortCriteria { field, direction }
}
