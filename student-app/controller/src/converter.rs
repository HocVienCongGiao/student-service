use crate::StudentCollectionQuery;
use chrono::{DateTime, Utc};
use domain::boundaries::shared_boundary::SortDirectionRequest;
use domain::boundaries::usecase_boundary::{
    StudentQueryUsecaseRequest, StudentSortCriteriaUsecaseRequest, StudentSortFieldUsecaseRequest,
    StudentSortUsecaseRequest,
};
use hvcg_academics_openapi_student::models::StudentSortCriteria;

impl StudentCollectionQuery {
    fn to_usecase_request(&self) -> StudentQueryUsecaseRequest {
        StudentQueryUsecaseRequest {
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
) -> Option<StudentSortUsecaseRequest> {
    let sort_request: Option<StudentSortUsecaseRequest>;
    let sort_criteria_dto = openapi;

    if let Some(sort_criteria_dto) = sort_criteria_dto {
        let mut sort_criteria = Vec::new();
        sort_criteria_dto.iter().for_each(|criterion| {
            let sort_criteria_request = match criterion {
                StudentSortCriteria::NAME_ASC => build_sort_criteria_request(
                    StudentSortFieldUsecaseRequest::Name,
                    SortDirectionRequest::Asc,
                ),
                StudentSortCriteria::NAME_DESC => build_sort_criteria_request(
                    StudentSortFieldUsecaseRequest::Name,
                    SortDirectionRequest::Desc,
                ),
                StudentSortCriteria::CHRISTIAN_NAME_ASC => build_sort_criteria_request(
                    StudentSortFieldUsecaseRequest::ChristianName,
                    SortDirectionRequest::Asc,
                ),
                StudentSortCriteria::CHRISTIAN_NAME_DESC => build_sort_criteria_request(
                    StudentSortFieldUsecaseRequest::ChristianName,
                    SortDirectionRequest::Desc,
                ),
                StudentSortCriteria::POLITY_NAME_ASC => build_sort_criteria_request(
                    StudentSortFieldUsecaseRequest::PolityName,
                    SortDirectionRequest::Asc,
                ),
                StudentSortCriteria::POLITY_NAME_DESC => build_sort_criteria_request(
                    StudentSortFieldUsecaseRequest::PolityName,
                    SortDirectionRequest::Desc,
                ),
                StudentSortCriteria::LOCATION_NAME_ASC => build_sort_criteria_request(
                    StudentSortFieldUsecaseRequest::LocationName,
                    SortDirectionRequest::Asc,
                ),
                StudentSortCriteria::LOCATION_NAME_DESC => build_sort_criteria_request(
                    StudentSortFieldUsecaseRequest::LocationName,
                    SortDirectionRequest::Desc,
                ),
                StudentSortCriteria::PLACE_OF_BIRTH_ASC => build_sort_criteria_request(
                    StudentSortFieldUsecaseRequest::PlaceOfBirth,
                    SortDirectionRequest::Asc,
                ),
                StudentSortCriteria::PLACE_OF_BIRTH_DESC => build_sort_criteria_request(
                    StudentSortFieldUsecaseRequest::PlaceOfBirth,
                    SortDirectionRequest::Desc,
                ),
            };
            sort_criteria.push(sort_criteria_request);
        });

        sort_request = Option::from(StudentSortUsecaseRequest { sort_criteria });
    } else {
        sort_request = None;
    }
    sort_request
}

fn build_sort_criteria_request(
    field: StudentSortFieldUsecaseRequest,
    direction: SortDirectionRequest,
) -> StudentSortCriteriaUsecaseRequest {
    StudentSortCriteriaUsecaseRequest { field, direction }
}
