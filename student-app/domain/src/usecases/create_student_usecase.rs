use async_trait::async_trait;
use uuid::Uuid;

use crate::entities::student::Student as StudentEntity;
use crate::ports::person::person_db_gateway::PersonDbGateway;
use crate::ports::student::student_db_gateway::StudentDbGateway;
use crate::usecases::{ToEntity, ToUsecaseOutput, UsecaseError};

pub struct CreateStudentUsecaseInteractor<A: StudentDbGateway, B: PersonDbGateway> {
    student_db_gateway: A,
    person_db_gateway: B,
}

impl<A, B> CreateStudentUsecaseInteractor<A, B>
where
    A: StudentDbGateway + Sync + Send,
    B: PersonDbGateway + Sync + Send,
{
    pub fn new(student_db_gateway: A, person_db_gateway: B) -> Self {
        CreateStudentUsecaseInteractor {
            student_db_gateway,
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
impl<A, B> CreateStudentUsecase for CreateStudentUsecaseInteractor<A, B>
where
    A: StudentDbGateway + Sync + Send,
    B: PersonDbGateway + Sync + Send,
{
    async fn execute(
        &mut self,
        request: CreateStudentUsecaseInput,
    ) -> Result<CreateStudentUsecaseOutput, UsecaseError> {
        let student = request.to_entity();
        if student.is_valid() {
            println!("This student is valid");

            let person = (*self)
                .person_db_gateway
                .find_one_by_id(student.person_id.unwrap())
                .await;
            if person.is_none() {
                eprintln!("This person id is not valid");
                Err(UsecaseError::ResourceNotFound)
            } else {
                let usecase_output = (*self)
                    .student_db_gateway
                    .insert(student)
                    .await
                    .map(|resp| resp.to_usecase_output())
                    .map_err(|err| err.to_usecase_error());

                usecase_output
            }
        } else {
            println!("This student is not valid");
            Err(UsecaseError::InvalidInput)
        }
    }
}

pub struct CreateStudentUsecaseInput {
    pub person_id: Uuid,
}

#[derive(Clone)]
pub struct CreateStudentUsecaseOutput {
    pub student_id: Uuid,
    pub person_id: Uuid,
}

impl ToEntity<StudentEntity> for CreateStudentUsecaseInput {
    fn to_entity(self) -> StudentEntity {
        StudentEntity {
            student_id: Some(Uuid::new_v4()),
            person_id: Some(self.person_id),
        }
    }
}

impl ToUsecaseOutput<CreateStudentUsecaseOutput> for StudentEntity {
    fn to_usecase_output(self) -> CreateStudentUsecaseOutput {
        CreateStudentUsecaseOutput {
            student_id: self.student_id.unwrap(),
            person_id: self.person_id.unwrap(),
        }
    }
}
