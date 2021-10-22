use crate::entities::student::Student;
use crate::ports::delete_student_port::DeleteStudentPort;
use crate::ports::find_one_student_by_id_port::FindOneStudentByIdPort;
use crate::ports::find_student_collection_port::FindStudentCollectionPort;
use crate::ports::insert_student_port::InsertStudentPort;
use crate::ports::update_student_port::UpdateStudentPort;
use crate::SortDirection;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
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
    pub date_of_birth: Option<DateTime<Utc>>,
    pub place_of_birth: Option<String>,
    pub polity_name: Option<String>,
    pub specialism: Option<String>,
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

pub enum StudentSortFieldDbRequest {
    FirstName,
    MiddleName,
    LastName,
}

pub struct StudentMutationDbRequest {
    pub id: Option<Uuid>,
    pub polity_id: Option<Uuid>,
    pub saint_ids: Option<Vec<uuid::Uuid>>,
    pub title: Option<String>,
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub last_name: Option<String>,
    pub date_of_birth: Option<DateTime<Utc>>,
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
    pub date_of_birth: Option<DateTime<Utc>>,
    pub place_of_birth: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub undergraduate_school: Option<String>,
}

pub struct StudentCollectionDbResponse {
    pub collection: Vec<StudentDbResponse>,
    pub has_more: bool,
    pub total: i64,
}

impl Student {
    pub fn to_mutation_db_request(&self) -> StudentMutationDbRequest {
        StudentMutationDbRequest {
            id: self.id,
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
