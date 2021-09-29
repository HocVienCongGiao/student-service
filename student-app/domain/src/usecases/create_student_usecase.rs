use crate::entities::student::{Student as StudentEntity, StudentTitle};
use crate::ports::student_db_gateway::{StudentDbGateway, StudentDbResponse};
use crate::usecases::student_usecase_shared_models::{
    StudentUsecaseSharedTitle, WithChristianName, WithPolity,
};
use crate::usecases::{ToEntity, ToUsecaseOutput, UsecaseError};
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
    pub polity_id: Option<Uuid>,
    pub saint_ids: Option<Vec<uuid::Uuid>>,
    pub title: Option<StudentUsecaseSharedTitle>,
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub last_name: Option<String>,
    pub date_of_birth: Option<DateTime<Utc>>,
    pub place_of_birth: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub undergraduate_school: Option<String>,
}

#[derive(Clone)]
pub struct CreateStudentUsecaseOutput {
    pub id: Uuid,
    pub polity_id: Option<Uuid>,
    pub polity_name: Option<String>,
    pub polity_location_name: Option<String>,
    pub polity_location_address: Option<String>,
    pub polity_location_email: Option<String>,
    pub saint_ids: Option<Vec<Uuid>>,
    pub christian_name: Option<String>,
    pub title: Option<StudentUsecaseSharedTitle>,
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
        let title_usecase_input = self.title;
        let mut title: Option<StudentTitle> = None;
        if let Some(title_usecase_input) = title_usecase_input {
            title = Some(title_usecase_input.to_entity());
        }
        StudentEntity {
            id: Some(Uuid::new_v4()),
            polity_id: self.polity_id,
            saint_ids: self.saint_ids,
            title,
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

impl ToEntity<StudentTitle> for StudentUsecaseSharedTitle {
    fn to_entity(self) -> StudentTitle {
        match self {
            StudentUsecaseSharedTitle::Monk => StudentTitle::Monk,
            StudentUsecaseSharedTitle::Nun => StudentTitle::Nun,
            StudentUsecaseSharedTitle::Priest => StudentTitle::Priest,
        }
    }
}

impl ToUsecaseOutput<CreateStudentUsecaseOutput> for StudentDbResponse {
    fn to_usecase_output(self) -> CreateStudentUsecaseOutput {
        CreateStudentUsecaseOutput {
            id: self.id,
            polity_id: self.polity_id,
            polity_name: None,
            polity_location_name: None,
            polity_location_address: None,
            polity_location_email: None,
            saint_ids: self.saint_ids.clone(),
            christian_name: None,
            title: self.title.map(|t| t.parse().unwrap()),
            first_name: self.first_name.clone(),
            middle_name: self.middle_name.clone(),
            last_name: self.last_name.clone(),
            date_of_birth: self.date_of_birth,
            place_of_birth: self.place_of_birth.clone(),
            email: self.email.clone(),
            phone: self.phone.clone(),
            undergraduate_school: self.undergraduate_school,
        }
    }
}

impl WithPolity<CreateStudentUsecaseOutput> for CreateStudentUsecaseOutput {
    fn with_polity(
        mut self,
        name: Option<String>,
        location_name: Option<String>,
        location_address: Option<String>,
        location_email: Option<String>,
    ) -> CreateStudentUsecaseOutput {
        self.polity_name = name;
        self.polity_location_name = location_name;
        self.polity_location_address = location_address;
        self.polity_location_email = location_email;
        self
    }
}

impl WithChristianName<CreateStudentUsecaseOutput> for CreateStudentUsecaseOutput {
    fn with_christian_name(mut self, name: Option<String>) -> CreateStudentUsecaseOutput {
        self.christian_name = name;
        self
    }
}
