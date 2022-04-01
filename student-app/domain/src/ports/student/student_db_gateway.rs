use crate::entities::student::Student;
use crate::ports::delete_student_port::DeleteStudentPort;
use crate::ports::find_one_student_by_id_port::FindOneStudentByIdPort;
use crate::ports::find_student_collection_port::FindStudentCollectionPort;
use crate::ports::student::insert_student_port::InsertStudentPort;
use crate::ports::student::models::student_dbrequest::{
    StudentQuery as StudentQueryDbRequest, StudentSort as StudentSortDbRequest,
    StudentSortCriteria as StudentSortCriteriaDbRequest,
    StudentSortField as StudentSortFieldDbRequest,
};
use crate::ports::student::models::student_mutation_dbrequest::Student as StudentMutationDbRequest;
use crate::ports::student::update_student_port::UpdateStudentPort;
use crate::usecases::query_student_collection_usecase::{
    QueryStudentCollectionUsecaseInput, QueryStudentCollectionUsecaseInputSort,
    QueryStudentCollectionUsecaseInputSortCriteria, QueryStudentCollectionUsecaseInputSortField,
};
use async_trait::async_trait;

#[async_trait]
pub trait StudentDbGateway:
    InsertStudentPort
    + UpdateStudentPort
    + DeleteStudentPort
    + FindStudentCollectionPort
    + FindOneStudentByIdPort
{
}

impl Student {
    pub fn to_mutation_db_request(&self) -> StudentMutationDbRequest {
        StudentMutationDbRequest {
            person_id: self.person_id,
            student_id: self.student_id,
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
