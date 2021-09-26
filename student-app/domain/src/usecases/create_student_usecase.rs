use crate::entities::student::Student as StudentEntity;
use crate::usecases::student_db_gateway::{StudentDbGateway, StudentDbResponse};
use crate::usecases::{ToEntity, UsecaseError};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct CreateStudentUsecaseInteractor<A: StudentDbGateway> {
    db_gateway: A,
}

impl<A> CreateStudentUsecaseInteractor<A>
where
    A: StudentDbGateway + Sync + Send,
{
    pub fn new(db_gateway: A) -> Self {
        CreateStudentUsecaseInteractor { db_gateway }
    }
}

#[async_trait]
pub trait CreateStudentUsecase {
    // InputBoundary
    async fn execute(
        &mut self,
        request: CreateStudentUsecaseInput,
    ) -> Result<CreateStudentUsecaseOutput, UsecaseError>;
    // async fn update_student(
    //     &mut self,
    //     request: StudentMutationUsecaseInput,
    // ) -> Result<StudentUsecaseOutput, UsecaseError>;
    // async fn delete_student(
    //     &mut self,
    //     request: StudentMutationUsecaseInput,
    // ) -> Result<(), UsecaseError>;
}

#[async_trait]
impl<A> CreateStudentUsecase for CreateStudentUsecaseInteractor<A>
where
    A: StudentDbGateway + Sync + Send,
{
    async fn execute(
        &mut self,
        request: CreateStudentUsecaseInput,
    ) -> Result<CreateStudentUsecaseOutput, UsecaseError> {
        let student = request.to_entity();
        if student.is_valid() {
            println!("This student is valid");
            (*self)
                .db_gateway
                .insert(student.to_mutation_db_request())
                .await
                .map(|response| response.to_usecase_output())
                .map_err(|err| err.to_usecase_error())
        } else {
            println!("This student is not valid");
            Err(UsecaseError::InvalidInput)
        }
    }

    // async fn update_student(
    //     &mut self,
    //     request: CreateStudentUsecaseInput,
    // ) -> Result<StudentUsecaseOutput, UsecaseError> {
    //     todo!()
    // }
    //
    // async fn delete_student(
    //     &mut self,
    //     request: CreateStudentUsecaseInput,
    // ) -> Result<(), UsecaseError> {
    //     todo!()
    // }
}

pub struct CreateStudentUsecaseInput {
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

pub struct CreateStudentUsecaseOutput {
    pub id: Option<Uuid>,
    pub polity_id: Option<Uuid>,
    pub saint_ids: Option<Vec<Uuid>>,
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

impl ToEntity<StudentEntity> for CreateStudentUsecaseInput {
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

impl StudentDbResponse {
    pub(crate) fn to_usecase_output(&self) -> CreateStudentUsecaseOutput {
        // let polity_usecase_output: Option<PolityUsecaseOutput>;
        // let polity_db_response = &self.polity;
        // if let Some(polity_db_response) = polity_db_response {
        //     polity_usecase_output = Option::from(polity_db_response.to_usecase_output());
        // } else {
        //     polity_usecase_output = None;
        // }
        CreateStudentUsecaseOutput {
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
