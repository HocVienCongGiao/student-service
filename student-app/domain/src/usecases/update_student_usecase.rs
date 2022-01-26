use async_trait::async_trait;
use chrono::NaiveDate;
use uuid::Uuid;

use crate::entities::person::{Person as PersonEntity, PersonTitle};
use crate::entities::student::Student as StudentEntity;
use crate::ports::person_db_gateway::{PersonDbGateway, PersonDbResponse};
use crate::ports::polity_db_gateway::PolityDbGateway;
use crate::ports::saint_db_gateway::SaintDbGateway;
use crate::ports::student_db_gateway::StudentDbGateway;
use crate::usecases::student_usecase_shared_models::{
    StudentUsecaseSharedTitle, WithChristianName, WithPolity, WithStudentId,
};
use crate::usecases::{ToEntity, ToUsecaseOutput, UsecaseError};

pub struct UpdateStudentUsecaseInteractor<
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

impl<A, B, C, D> UpdateStudentUsecaseInteractor<A, B, C, D>
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
        UpdateStudentUsecaseInteractor {
            student_db_gateway,
            polity_db_gateway,
            saint_db_gateway,
            person_db_gateway,
        }
    }
}

#[async_trait]
pub trait UpdateStudentUsecase {
    // InputBoundary
    async fn execute(
        &mut self,
        request: UpdateStudentUsecaseInput,
    ) -> Result<UpdateStudentUsecaseOutput, UsecaseError>;
}

#[async_trait]
impl<A, B, C, D> UpdateStudentUsecase for UpdateStudentUsecaseInteractor<A, B, C, D>
where
    A: StudentDbGateway + Sync + Send,
    B: PolityDbGateway + Sync + Send,
    C: SaintDbGateway + Sync + Send,
    D: PersonDbGateway + Sync + Send,
{
    async fn execute(
        &mut self,
        request: UpdateStudentUsecaseInput,
    ) -> Result<UpdateStudentUsecaseOutput, UsecaseError> {
        // id not exists
        if request.student_id.is_none() {
            println!("student not exists");
            return Err(UsecaseError::ResourceNotFound);
        }
        let student_db_response = (*self)
            .student_db_gateway
            .find_one_by_id(request.student_id.unwrap())
            .await;
        if student_db_response.is_none() {
            println!("student not exists");
            return Err(UsecaseError::ResourceNotFound);
        }

        // id exists
        let student = request.to_entity();
        if student.is_valid() {
            println!("This student is valid");
            let student_id = student.student_id.unwrap();

            let person_id = (*self)
                .student_db_gateway
                .find_person_id_by_student_id(student_id)
                .await
                .map_err(|err| err.to_usecase_error())?;

            let person = student.to_person_entity(person_id);

            let usecase_output: Result<UpdateStudentUsecaseOutput, UsecaseError> = (*self)
                .person_db_gateway
                .update(person.to_mutation_db_request())
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
                    output = output.with_student_id(Some(student_id));
                    Ok(output)
                }
                Err(error) => {
                    println!("Update fail");
                    Err(error)
                }
            };
        } else {
            println!("This student is not valid");
            Err(UsecaseError::InvalidInput)
        }
    }
}

pub struct UpdateStudentUsecaseInput {
    pub student_id: Option<Uuid>,
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
pub struct UpdateStudentUsecaseOutput {
    pub student_id: Option<Uuid>,
    pub person_id: Option<Uuid>,
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

impl ToEntity<StudentEntity> for UpdateStudentUsecaseInput {
    fn to_entity(self) -> StudentEntity {
        let title_usecase_input = self.title;
        let mut title: Option<PersonTitle> = None;
        if let Some(title_usecase_input) = title_usecase_input {
            title = Some(title_usecase_input.to_entity());
        }
        let person = PersonEntity {
            id: None,
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
        };
        StudentEntity {
            person: Some(person),
            student_id: self.student_id,
        }
    }
}

impl ToUsecaseOutput<UpdateStudentUsecaseOutput> for PersonDbResponse {
    fn to_usecase_output(self) -> UpdateStudentUsecaseOutput {
        UpdateStudentUsecaseOutput {
            student_id: None,
            person_id: Some(self.id),
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
            phone: self.phone,
        }
    }
}

impl WithPolity<UpdateStudentUsecaseOutput> for UpdateStudentUsecaseOutput {
    fn with_polity(
        mut self,
        name: Option<String>,
        location_name: Option<String>,
        location_address: Option<String>,
        location_email: Option<String>,
    ) -> UpdateStudentUsecaseOutput {
        self.polity_name = name;
        self.polity_location_name = location_name;
        self.polity_location_address = location_address;
        self.polity_location_email = location_email;
        self
    }
}

impl WithChristianName<UpdateStudentUsecaseOutput> for UpdateStudentUsecaseOutput {
    fn with_christian_name(mut self, name: Option<String>) -> UpdateStudentUsecaseOutput {
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

impl UpdateStudentUsecaseInput {
    pub fn with_student_id(mut self, id: Uuid) -> UpdateStudentUsecaseInput {
        self.student_id = Some(id);
        self
    }
}

impl StudentEntity {
    pub fn to_person_entity(&self, id: Uuid) -> PersonEntity {
        let mut person = self.person.clone().unwrap();
        person.id = Some(id);
        person
    }
}

impl WithStudentId<UpdateStudentUsecaseOutput> for UpdateStudentUsecaseOutput {
    fn with_student_id(mut self, student_id: Option<Uuid>) -> UpdateStudentUsecaseOutput {
        if let Some(student_id) = student_id {
            self.student_id = Some(student_id)
        }
        self
    }
}
