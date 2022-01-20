use crate::entities::student::Student;
use crate::ports::delete_student_port::DeleteStudentPort;
use crate::ports::find_one_student_by_id_port::FindOneStudentByIdPort;
use crate::ports::find_student_collection_port::FindStudentCollectionPort;
use crate::ports::insert_student_port::InsertStudentPort;
use crate::ports::update_student_port::UpdateStudentPort;
use crate::usecases::query_student_collection_usecase::{
    QueryStudentCollectionUsecaseInput, QueryStudentCollectionUsecaseInputSort,
    QueryStudentCollectionUsecaseInputSortCriteria, QueryStudentCollectionUsecaseInputSortField,
};
use crate::SortDirection;
use async_trait::async_trait;
use chrono::NaiveDate;
use uuid::Uuid;

#[async_trait]
pub trait StudentDbGateway:
    InsertStudentPort
    + UpdateStudentPort
    + DeleteStudentPort
    + FindStudentCollectionPort
    + FindOneStudentByIdPort
{
}

pub struct StudentQueryDbRequest {
    pub id: Option<Uuid>,
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub undergraduate_school: Option<String>,
    pub date_of_birth: Option<NaiveDate>,
    pub place_of_birth: Option<String>,
    pub polity_name: Option<String>,
    //pub specialism: Option<String>,
    pub sort_request: Option<StudentSortDbRequest>,
    pub offset: Option<i64>,
    pub count: Option<i64>,
}

pub struct StudentSortDbRequest {
    pub sort_criteria: Vec<StudentSortCriteriaDbRequest>,
}

pub struct StudentSortCriteriaDbRequest {
    pub field: StudentSortFieldDbRequest,
    pub direction: SortDirection,
}
#[derive(strum_macros::Display)]
pub enum StudentSortFieldDbRequest {
    FirstName,
    MiddleName,
    LastName,
    ChristianName,
    PolityName,
    LocationName,
    PlaceOfBirth,
}

pub struct StudentMutationDbRequest {
    // TODO: ask do we need to split person_id and student_id into 2 DbRequest
    pub person_id: Option<Uuid>,
    pub student_id: Option<Uuid>,
    pub polity_id: Option<Uuid>,
    pub saint_ids: Option<Vec<uuid::Uuid>>,
    pub title: Option<String>,
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub last_name: Option<String>,
    pub date_of_birth: Option<NaiveDate>,
    pub place_of_birth: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub undergraduate_school: Option<String>,
}

pub struct StudentDbResponse {
    pub id: Uuid,
    pub polity_id: Option<uuid::Uuid>,
    pub saint_ids: Option<Vec<uuid::Uuid>>,
    pub title: Option<String>,
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub last_name: Option<String>,
    pub date_of_birth: Option<NaiveDate>,
    pub place_of_birth: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub undergraduate_school: Option<String>,
}

pub struct StudentCollectionDbResponse {
    pub collection: Vec<StudentDbResponse>,
    pub has_more: Option<bool>,
    pub total: i64,
}

impl Student {
    pub fn to_mutation_db_request(&self) -> StudentMutationDbRequest {
        StudentMutationDbRequest {
            person_id: self.person_id,
            student_id: self.student_id,
            polity_id: self.polity_id,
            saint_ids: self.saint_ids.clone(),
            title: self.title.clone().map(|title| title.to_string()),
            first_name: self.first_name.clone(),
            middle_name: self.middle_name.clone(),
            last_name: self.last_name.clone(),
            date_of_birth: self.date_of_birth,
            place_of_birth: self.place_of_birth.clone(),
            email: self.email.clone(),
            phone: self.phone.clone(),
            undergraduate_school: self.undergraduate_school.clone(),
        }
    }
}

impl QueryStudentCollectionUsecaseInput {
    pub fn to_query_db_request(&self) -> StudentQueryDbRequest {
        StudentQueryDbRequest {
            id: self.id,
            name: self.name.clone(),
            email: self.email.clone(),
            phone: self.phone.clone(),
            undergraduate_school: self.undergraduate_school.clone(),
            date_of_birth: self.date_of_birth,
            place_of_birth: self.place_of_birth.clone(),
            polity_name: self.polity_name.clone(),
            //specialism: self.specialism,
            sort_request: self
                .sort_request
                .as_ref()
                .map(|input_sort| input_sort.to_student_sort_db_request()),
            offset: self.offset,
            count: self.count,
        }
    }
}

impl QueryStudentCollectionUsecaseInputSort {
    fn to_student_sort_db_request(&self) -> StudentSortDbRequest {
        let sort_criteria_db_request = self
            .sort_criteria
            .iter()
            .map(|criterion| criterion.to_student_sort_criteria_db_request())
            .collect();
        StudentSortDbRequest {
            sort_criteria: sort_criteria_db_request,
        }
    }
}

impl QueryStudentCollectionUsecaseInputSortCriteria {
    fn to_student_sort_criteria_db_request(&self) -> StudentSortCriteriaDbRequest {
        let field = &self.field;

        StudentSortCriteriaDbRequest {
            field: match field {
                QueryStudentCollectionUsecaseInputSortField::FirstName => {
                    StudentSortFieldDbRequest::FirstName
                }
                QueryStudentCollectionUsecaseInputSortField::MiddleName => {
                    StudentSortFieldDbRequest::MiddleName
                }
                QueryStudentCollectionUsecaseInputSortField::LastName => {
                    StudentSortFieldDbRequest::LastName
                }
                QueryStudentCollectionUsecaseInputSortField::ChristianName => {
                    StudentSortFieldDbRequest::ChristianName
                }
                QueryStudentCollectionUsecaseInputSortField::PolityName => {
                    StudentSortFieldDbRequest::PolityName
                }
                QueryStudentCollectionUsecaseInputSortField::LocationName => {
                    StudentSortFieldDbRequest::LocationName
                }
                QueryStudentCollectionUsecaseInputSortField::PlaceOfBirth => {
                    StudentSortFieldDbRequest::PlaceOfBirth
                }
            },
            direction: self.direction.clone(),
        }
    }
}
