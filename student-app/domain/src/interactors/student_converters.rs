use crate::boundaries::db_gateway_boundary::{
    DbError, PolityDbResponse, StudentDbResponse, StudentMutationDbRequest,
};
use crate::boundaries::usecase_boundary::{
    PolityUsecaseResponse, StudentMutationUsecaseRequest, StudentUsecaseResponse, UsecaseError,
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
        let polity_usecase_response: Option<PolityUsecaseResponse>;
        let polity_db_response = &self.polity;
        if let Some(polity_db_response) = polity_db_response {
            polity_usecase_response = Option::from(polity_db_response.to_usecase_response());
        } else {
            polity_usecase_response = None;
        }
        StudentUsecaseResponse {
            id: self.id,
            polity: polity_usecase_response,
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

impl PolityDbResponse {
    pub(crate) fn to_usecase_response(&self) -> PolityUsecaseResponse {
        PolityUsecaseResponse {
            id: self.id,
            name: self.name.clone(),
            location_name: self.location_name.clone(),
            location_address: self.location_address.clone(),
            location_email: self.location_email.clone(),
        }
    }
}
