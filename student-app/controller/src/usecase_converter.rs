use crate::StudentCollectionQuery;
use chrono::{DateTime, Utc};
use domain::boundaries::shared_boundary::SortDirectionRequest;
use domain::boundaries::usecase_boundary::{
    StudentQueryUsecaseInput, StudentSortCriteriaUsecaseInput, StudentSortFieldUsecaseInput,
    StudentSortUsecaseInput,
};
use hvcg_academics_openapi_student::models::StudentSortCriteria;

impl StudentCollectionQuery {
    pub fn to_usecase_request(&self) -> StudentQueryUsecaseInput {
        StudentQueryUsecaseInput {
            id: None,
            name: self.name.clone(),
            email: self.email.clone(),
            phone: self.phone.clone(),
            undergraduate_school: self.undergraduate_school.clone(),
            date_of_birth: self.date_of_birth,
            place_of_birth: self.place_of_birth.clone(),
            polity_name: self.polity_name.clone(),
            specialism: self.specialism.clone(),
            sort_request: from_openapi_to_usecase_request(self.sorts.clone()),
            offset: self.offset,
            count: self.count,
        }
    }
}

pub fn from_openapi_to_usecase_request(
    openapi: Option<Vec<StudentSortCriteria>>,
) -> Option<StudentSortUsecaseInput> {
    let sort_request: Option<StudentSortUsecaseInput>;
    let sort_criteria_dto = openapi;

    if let Some(sort_criteria_dto) = sort_criteria_dto {
        let mut sort_criteria = Vec::new();
        sort_criteria_dto.iter().for_each(|criterion| {
            let sort_criteria_request = match criterion {
                StudentSortCriteria::NAME_ASC => build_sort_criteria_request(
                    StudentSortFieldUsecaseInput::Name,
                    SortDirectionRequest::Asc,
                ),
                StudentSortCriteria::NAME_DESC => build_sort_criteria_request(
                    StudentSortFieldUsecaseInput::Name,
                    SortDirectionRequest::Desc,
                ),
                StudentSortCriteria::CHRISTIAN_NAME_ASC => build_sort_criteria_request(
                    StudentSortFieldUsecaseInput::ChristianName,
                    SortDirectionRequest::Asc,
                ),
                StudentSortCriteria::CHRISTIAN_NAME_DESC => build_sort_criteria_request(
                    StudentSortFieldUsecaseInput::ChristianName,
                    SortDirectionRequest::Desc,
                ),
                StudentSortCriteria::POLITY_NAME_ASC => build_sort_criteria_request(
                    StudentSortFieldUsecaseInput::PolityName,
                    SortDirectionRequest::Asc,
                ),
                StudentSortCriteria::POLITY_NAME_DESC => build_sort_criteria_request(
                    StudentSortFieldUsecaseInput::PolityName,
                    SortDirectionRequest::Desc,
                ),
                StudentSortCriteria::LOCATION_NAME_ASC => build_sort_criteria_request(
                    StudentSortFieldUsecaseInput::LocationName,
                    SortDirectionRequest::Asc,
                ),
                StudentSortCriteria::LOCATION_NAME_DESC => build_sort_criteria_request(
                    StudentSortFieldUsecaseInput::LocationName,
                    SortDirectionRequest::Desc,
                ),
                StudentSortCriteria::PLACE_OF_BIRTH_ASC => build_sort_criteria_request(
                    StudentSortFieldUsecaseInput::PlaceOfBirth,
                    SortDirectionRequest::Asc,
                ),
                StudentSortCriteria::PLACE_OF_BIRTH_DESC => build_sort_criteria_request(
                    StudentSortFieldUsecaseInput::PlaceOfBirth,
                    SortDirectionRequest::Desc,
                ),
            };
            sort_criteria.push(sort_criteria_request);
        });

        sort_request = Option::from(StudentSortUsecaseInput { sort_criteria });
    } else {
        sort_request = None;
    }
    sort_request
}

fn build_sort_criteria_request(
    field: StudentSortFieldUsecaseInput,
    direction: SortDirectionRequest,
) -> StudentSortCriteriaUsecaseInput {
    StudentSortCriteriaUsecaseInput { field, direction }
}
