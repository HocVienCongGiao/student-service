use async_trait::async_trait;
use chrono::NaiveDate;
use uuid::Uuid;

use crate::entities::student::{Student as StudentEntity, StudentTitle};
use crate::ports::person_db_gateway::PersonDbGateway;
use crate::ports::polity_db_gateway::PolityDbGateway;
use crate::ports::saint_db_gateway::SaintDbGateway;
use crate::ports::student_db_gateway::{StudentDbGateway, StudentDbResponse};
use crate::usecases::student_usecase_shared_models::{
    StudentUsecaseSharedTitle, WithChristianName, WithPolity,
};
use crate::usecases::{ToEntity, ToUsecaseOutput, UsecaseError};

pub struct CreateStudentUsecaseInteractor<
    A: StudentDbGateway,
    B: PolityDbGateway,
    C: SaintDbGateway,
    D: PersonDbGateway,
> {
    student_db_gateway: A,
    polity_db_gateway: B,
    saint_db_gateway: C,
    person_db_gateway: D,
}

impl<A, B, C, D> CreateStudentUsecaseInteractor<A, B, C, D>
where
    A: StudentDbGateway + Sync + Send,
    B: PolityDbGateway + Sync + Send,
    C: SaintDbGateway + Sync + Send,
    D: PersonDbGateway + Sync + Send,
{
    pub fn new(
        student_db_gateway: A,
        polity_db_gateway: B,
        saint_db_gateway: C,
        person_db_gateway: D,
    ) -> Self {
        CreateStudentUsecaseInteractor {
            student_db_gateway,
            polity_db_gateway,
            saint_db_gateway,
            person_db_gateway,
        }
    }
}

#[async_trait]
pub trait CreateStudentUsecase {
    // InputBoundary
    async fn execute(
        &mut self,
        request: CreateStudentUsecaseInput,
    ) -> Result<CreateStudentUsecaseOutput, UsecaseError>;
}

#[async_trait]
impl<A, B, C, D> CreateStudentUsecase for CreateStudentUsecaseInteractor<A, B, C, D>
where
    A: StudentDbGateway + Sync + Send,
    B: PolityDbGateway + Sync + Send,
    C: SaintDbGateway + Sync + Send,
    D: PersonDbGateway + Sync + Send,
{
    async fn execute(
        &mut self,
        request: CreateStudentUsecaseInput,
    ) -> Result<CreateStudentUsecaseOutput, UsecaseError> {
        // TODO: cheat convert student -> person
        let student = request.to_entity();
        if student.is_valid() {
            println!("This student is valid");
            let usecase_output: Result<CreateStudentUsecaseOutput, UsecaseError> = (*self)
                .student_db_gateway
                .insert(student.to_mutation_db_request())
                .await
                .map(|response| response.to_usecase_output())
                .map_err(|err| err.to_usecase_error());

            return match usecase_output {
                Ok(output) => {
                    let mut output = output.with_polity(
                        Some("1".to_string()),
                        Some("1".to_string()),
                        Some("1".to_string()),
                        Some("1".to_string()),
                    );

                    if let Some(polity_id) = output.polity_id {
                        if let Some(polity_db_response) =
                            (*self).polity_db_gateway.find_one_by_id(polity_id).await
                        {
                            output = output.with_polity(
                                polity_db_response.name,
                                polity_db_response.location_name,
                                polity_db_response.location_address,
                                polity_db_response.location_email,
                            )
                        }
                    }
                    let saint_ids = output.saint_ids.clone();
                    if let Some(saint_ids) = saint_ids {
                        for (_i, e) in saint_ids.iter().enumerate() {
                            if let Some(saint_db_response) =
                                (*self).saint_db_gateway.find_one_by_id(*e).await
                            {
                                output = output.with_christian_name(saint_db_response.display_name)
                            }
                        }
                    }
                    Ok(output)
                }
                Err(error) => {
                    println!("Create fail");
                    Err(error)
                }
            };
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
    pub date_of_birth: Option<NaiveDate>,
    pub place_of_birth: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
}

#[derive(Clone)]
pub struct CreateStudentUsecaseOutput {
    pub person_id: Uuid,
    pub student_id: Option<Uuid>,
    pub polity_id: Option<Uuid>,
    pub polity_name: Option<String>,
    pub polity_location_name: Option<String>,
    pub polity_location_address: Option<String>,
    pub polity_location_email: Option<String>,
    pub saint_ids: Option<Vec<Uuid>>,
    pub christian_name: Option<Vec<String>>,
    pub title: Option<StudentUsecaseSharedTitle>,
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub last_name: Option<String>,
    pub date_of_birth: Option<NaiveDate>,
    pub place_of_birth: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
}

impl ToEntity<StudentEntity> for CreateStudentUsecaseInput {
    fn to_entity(self) -> StudentEntity {
        let title_usecase_input = self.title;
        let mut title: Option<StudentTitle> = None;
        if let Some(title_usecase_input) = title_usecase_input {
            title = Some(title_usecase_input.to_entity());
        }
        StudentEntity {
            person_id: Some(Uuid::new_v4()),
            student_id: Some(Uuid::new_v4()),
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
            undergraduate_school: None,
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
            person_id: self.id,
            student_id: None,
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
        if let Some(name) = name {
            let mut saint_names: Vec<String>;
            if self.christian_name.is_none() {
                saint_names = vec![];
            } else {
                saint_names = self.christian_name.unwrap();
            }
            saint_names.push(name);
            self.christian_name = Some(saint_names);
        }
        self
    }
}
