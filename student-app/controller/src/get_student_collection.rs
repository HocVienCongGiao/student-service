use crate::openapi::ToOpenApi;
use crate::StudentCollectionQuery;
use db_postgres::polity_gateway::repository::PolityRepository;
use db_postgres::saint_gateway::repository::SaintRepository;
use db_postgres::student_gateway::repository::StudentRepository;
use domain::usecases::query_student_collection_usecase::{
    QueryStudentCollectionUsecase, QueryStudentCollectionUsecaseInput,
    QueryStudentCollectionUsecaseInputSort, QueryStudentCollectionUsecaseInputSortCriteria,
    QueryStudentCollectionUsecaseInputSortField, QueryStudentCollectionUsecaseInteractor,
};
use domain::SortDirection;
use hvcg_academics_openapi_student::models::{StudentSortCriteria, StudentViewCollection};

pub(crate) async fn from_usecase_input(
    request: QueryStudentCollectionUsecaseInput,
) -> StudentViewCollection {
    // Init dependencies
    let client = db_postgres::connect().await;
    let student_repository = StudentRepository { client };

    let polity_client = db_postgres::connect().await;
    let polity_repository = PolityRepository {
        client: polity_client,
    };

    let saint_client = db_postgres::connect().await;
    let saint_repository = SaintRepository {
        client: saint_client,
    };

    // Inject dependencies to Interactor and invoke func
    let query_students_usecase_output = QueryStudentCollectionUsecaseInteractor::new(
        student_repository,
        polity_repository,
        saint_repository,
    )
    .execute(request)
    .await;

    query_students_usecase_output.to_openapi()
}

impl StudentCollectionQuery {
    pub fn to_usecase_input(&self) -> QueryStudentCollectionUsecaseInput {
        QueryStudentCollectionUsecaseInput {
            id: None,
            name: self.name.clone(),
            email: self.email.clone(),
            phone: self.phone.clone(),
            undergraduate_school: self.undergraduate_school.clone(),
            date_of_birth: self.date_of_birth,
            place_of_birth: self.place_of_birth.clone(),
            polity_name: self.polity_name.clone(),
            //specialism: self.specialism.clone(),
            sort_request: from_openapi_to_usecase_input(self.sorts.clone()),
            offset: self.offset,
            count: self.count,
        }
    }
}

pub fn from_openapi_to_usecase_input(
    openapi: Option<Vec<StudentSortCriteria>>,
) -> Option<QueryStudentCollectionUsecaseInputSort> {
    let sort_criteria_dto = openapi;

    let sort_request: Option<QueryStudentCollectionUsecaseInputSort> =
        if let Some(sort_criteria_dto) = sort_criteria_dto {
            let mut sort_criteria = Vec::new();
            sort_criteria_dto.iter().for_each(|criterion| {
                let sort_criteria_request = match criterion {
                    StudentSortCriteria::NAME_ASC => build_sort_criteria_request(
                        QueryStudentCollectionUsecaseInputSortField::LastName,
                        SortDirection::Asc,
                    ),
                    StudentSortCriteria::NAME_DESC => build_sort_criteria_request(
                        QueryStudentCollectionUsecaseInputSortField::LastName,
                        SortDirection::Desc,
                    ),
                    StudentSortCriteria::CHRISTIAN_NAME_ASC => build_sort_criteria_request(
                        QueryStudentCollectionUsecaseInputSortField::ChristianName,
                        SortDirection::Asc,
                    ),
                    StudentSortCriteria::CHRISTIAN_NAME_DESC => build_sort_criteria_request(
                        QueryStudentCollectionUsecaseInputSortField::ChristianName,
                        SortDirection::Desc,
                    ),
                    StudentSortCriteria::POLITY_NAME_ASC => build_sort_criteria_request(
                        QueryStudentCollectionUsecaseInputSortField::PolityName,
                        SortDirection::Asc,
                    ),
                    StudentSortCriteria::POLITY_NAME_DESC => build_sort_criteria_request(
                        QueryStudentCollectionUsecaseInputSortField::PolityName,
                        SortDirection::Desc,
                    ),
                    StudentSortCriteria::LOCATION_NAME_ASC => build_sort_criteria_request(
                        QueryStudentCollectionUsecaseInputSortField::LocationName,
                        SortDirection::Asc,
                    ),
                    StudentSortCriteria::LOCATION_NAME_DESC => build_sort_criteria_request(
                        QueryStudentCollectionUsecaseInputSortField::LocationName,
                        SortDirection::Desc,
                    ),
                    StudentSortCriteria::PLACE_OF_BIRTH_ASC => build_sort_criteria_request(
                        QueryStudentCollectionUsecaseInputSortField::PlaceOfBirth,
                        SortDirection::Asc,
                    ),
                    StudentSortCriteria::PLACE_OF_BIRTH_DESC => build_sort_criteria_request(
                        QueryStudentCollectionUsecaseInputSortField::PlaceOfBirth,
                        SortDirection::Desc,
                    ),
                };

                let sort_field = sort_criteria_request.field.clone();
                let sort_direction = sort_criteria_request.direction.clone();

                sort_criteria.push(sort_criteria_request);

                if QueryStudentCollectionUsecaseInputSortField::LastName == sort_field {
                    sort_criteria.push(build_sort_criteria_request(
                        QueryStudentCollectionUsecaseInputSortField::MiddleName,
                        sort_direction.clone(),
                    ));
                    sort_criteria.push(build_sort_criteria_request(
                        QueryStudentCollectionUsecaseInputSortField::FirstName,
                        sort_direction,
                    ));
                }
            });

            Option::from(QueryStudentCollectionUsecaseInputSort { sort_criteria })
        } else {
            None
        };
    sort_request
}

fn build_sort_criteria_request(
    field: QueryStudentCollectionUsecaseInputSortField,
    direction: SortDirection,
) -> QueryStudentCollectionUsecaseInputSortCriteria {
    QueryStudentCollectionUsecaseInputSortCriteria { field, direction }
}
