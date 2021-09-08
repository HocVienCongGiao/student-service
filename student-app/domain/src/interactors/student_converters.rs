use crate::boundaries::db_gateway_boundary::{
    DbError, StudentDbResponse, StudentMutationDbRequest,
};
use crate::boundaries::usecase_boundary::{
    StudentMutationUsecaseRequest, StudentUsecaseResponse, UsecaseError,
};
use crate::entity::student::Student as StudentEntity;
use crate::interactors::ToEntity;

impl DbError {
    pub(crate) fn to_usecase_error(&self) -> UsecaseError {
        match self {
            DbError::UniqueConstraintViolationError(field) => {
                UsecaseError::UniqueConstraintViolationError(field.to_string())
            }
            DbError::UnknownError(msg) => UsecaseError::UnknownError(msg.to_string()),
        }
    }
}

impl ToEntity<StudentEntity> for StudentMutationUsecaseRequest {
    fn to_entity(self) -> StudentEntity {
        StudentEntity {
            id: self.id,
            polity_id: self.polity_id,
            saint_ids: self.saint_ids,
            title: self.title,
            first_name: self.first_name,
            middle_name: self.middle_name,
            last_name: self.last_name,
            date_of_birth: self.date_of_birth,
            place_of_birth: self.place_of_birth,
            email: self.email,
            phone: self.phone,
            undergraduate_school: self.undergraduate_school,
        }
    }
}

impl StudentEntity {
    pub(crate) fn to_mutation_db_request(&self) -> StudentMutationDbRequest {
        StudentMutationDbRequest {
            id: self.id,
            polity_id: self.polity_id,
            saint_ids: self.saint_ids.clone(),
            title: self.title.clone(),
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

impl StudentDbResponse {
    pub(crate) fn to_usecase_response(&self) -> StudentUsecaseResponse {
        StudentUsecaseResponse {
            id: self.id,
            polity_id: self.polity_id,
            saint_ids: self.saint_ids.clone(),
            title: self.title.clone(),
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
