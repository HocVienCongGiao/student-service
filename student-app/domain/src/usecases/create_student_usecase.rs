use async_trait::async_trait;
use chrono::{DateTime, NaiveDate, Utc};
use uuid::Uuid;

use crate::entities::person::{Person as PersonEntity, PersonTitle};
use crate::entities::person::{Person, VowProgress};
use crate::entities::person_educational_stage::{
    EducationalLevel, EducationalStage as EducationalStageEntity, EducationalStage,
};
use crate::entities::person_id_number::{
    PersonIdNumberProvider as PersonIdNumberProviderEntity, PersonIdNumberProvider,
};
use crate::entities::student::Student as StudentEntity;
use crate::ports::person_db_gateway::{PersonDbGateway, PersonDbResponse};
use crate::ports::polity_db_gateway::PolityDbGateway;
use crate::ports::saint_db_gateway::SaintDbGateway;
use crate::ports::student_db_gateway::{StudentDbGateway, StudentDbResponse};
use crate::ports::DbError;
use crate::usecases::person_usecase_shared_models::{
    PersonUsecaseSharedEducationalStage, PersonUsecaseSharedIdNumberProvider,
    PersonUsecaseSharedLanguage, PersonUsecaseSharedTitle, PersonUsecaseSharedVowProgress,
};
use crate::usecases::student_usecase_shared_models::{WithChristianName, WithPolity};
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
        let person = request.to_entity();
        // let educational_stages = request.to_educational_stage_entity();
        if person.is_valid() {
            println!("This student is valid");
            let usecase_output: Result<CreateStudentUsecaseOutput, UsecaseError> = (*self)
                .person_db_gateway
                .insert(person.to_mutation_db_request())
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
                    // TODO: refactor, get from view
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
            println!("This person is not valid");
            Err(UsecaseError::InvalidInput)
        }

        // if !educational_stages.is_empty() {
        //     println!("Education list is not empty");
        //     let insert_educational_stage: Result<EducationalStageDbResponse, UsecaseError> =
        //         (*self)
        //             .person_db_gateway
        //             .insert_educational_stage(educational_stages)
        //             .await
        //             .map_err(|err| err.to_usecase_error());
        // }

        // TODO: if insert education_stage, student not error
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
    pub saint_ids: Option<Vec<Uuid>>,
    pub title: Option<PersonUsecaseSharedTitle>,
    pub vow_progress: Option<PersonUsecaseSharedVowProgress>,
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub last_name: Option<String>,
    pub date_of_birth: Option<NaiveDate>,
    pub place_of_birth: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub educational_stages: Option<Vec<PersonUsecaseSharedEducationalStage>>,
    pub nationality: Option<String>,
    pub race: Option<String>,
    pub id_number: Option<String>,
    pub id_number_provider: Option<PersonUsecaseSharedIdNumberProvider>,
    pub date_of_issue: Option<NaiveDate>,
    pub place_of_issue: Option<String>,
    pub address: Option<String>,
    pub languages: Option<Vec<PersonUsecaseSharedLanguage>>,
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
    pub title: Option<String>,
    pub vow_progress: Option<String>,
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub last_name: Option<String>,
    pub date_of_birth: Option<NaiveDate>,
    pub place_of_birth: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub nationality: Option<String>,
    pub race: Option<String>,
    pub id_number: Option<String>,
    pub id_number_provider: Option<String>,
    pub date_of_issue: Option<NaiveDate>,
    pub place_of_issue: Option<String>,
    pub address: Option<String>,
    // pub languages: Option<Vec<Language>>, // todo()!
    // pub educational_stages: Option<Vec<EducationalStage>>, // todo()!
}

impl ToEntity<PersonEntity> for CreateStudentUsecaseInput {
    fn to_entity(self) -> PersonEntity {
        let title_usecase_input = self.title;
        let mut title: Option<PersonTitle> = None;
        if let Some(title_usecase_input) = title_usecase_input {
            title = Some(title_usecase_input.to_entity());
        }

        let vow_progress_usecase_input = self.vow_progress;
        let mut vow_progress: Option<VowProgress> = None;
        if let Some(vow_progress_usecase_input) = vow_progress_usecase_input {
            vow_progress = Some(vow_progress_usecase_input.to_entity());
        }

        // let mut language_list: Vec<Language> = Vec::new();
        // for language in self.language.unwrap() {
        //     language_list.push(LanguageEntity {
        //         person_id,
        //         language: language.language,
        //         level: language.level,
        //     })
        // }

        PersonEntity {
            id: Some(Uuid::new_v4()),
            student_id: Some(Uuid::new_v4()),
            polity_id: self.polity_id,
            saint_ids: self.saint_ids,
            title,
            vow_progress,
            first_name: self.first_name,
            middle_name: self.middle_name,
            last_name: self.last_name,
            date_of_birth: self.date_of_birth,
            place_of_birth: self.place_of_birth,
            date_of_issue: self.date_of_issue,
            place_of_issue: self.place_of_issue,
            email: self.email,
            phone: self.phone,
            nationality: self.nationality,
            race: self.race,
            address: self.address,
            // languages: language_list, // todo()!
        }
    }

    // fn to_educational_stage_entity(self, person_id: Uuid) -> Vec<EducationalStageEntity> {
    //     let mut educational_stage_list: Vec<EducationalStage> = Vec::new();
    //     for stage in self.educational_stages.unwrap() {
    //         let education_level_usecase_input = stage.educational_level;
    //         let mut educational_level: Option<EducationalLevel> = None;
    //         if let Some(education_level_usecase_input) = education_level_usecase_input {
    //             educational_level = Some(educational_level.to_entity());
    //         }
    //         educational_stage_list.push(EducationalStageEntity {
    //             id: Some(Uuid::new_v4()),
    //             educational_level: educational_level,
    //             school_name: stage.school_name,
    //             major: stage.major,
    //             graduate_year: stage.graduate_year,
    //             person_id: person_id,
    //         });
    //     }
    //     educational_stage_list
    // }
}

impl ToEntity<PersonTitle> for PersonUsecaseSharedTitle {
    fn to_entity(self) -> PersonTitle {
        match self {
            PersonUsecaseSharedTitle::Monk => PersonTitle::Monk,
            PersonUsecaseSharedTitle::Nun => PersonTitle::Nun,
            PersonUsecaseSharedTitle::Priest => PersonTitle::Priest,
        }
    }
}

impl ToEntity<VowProgress> for PersonUsecaseSharedVowProgress {
    fn to_entity(self) -> VowProgress {
        match self {
            PersonUsecaseSharedVowProgress::SolemnVow => VowProgress::SolemnVow,
            PersonUsecaseSharedVowProgress::SimpleVow => VowProgress::SimpleVow,
            PersonUsecaseSharedVowProgress::Novice => VowProgress::Novice,
            PersonUsecaseSharedVowProgress::Preparation => VowProgress::Preparation,
        }
    }
}

impl ToEntity<EducationalLevel> for EducationalLevel {
    fn to_entity(self) -> EducationalLevel {
        match self {
            EducationalLevel::ElementarySchool => EducationalLevel::ElementarySchool,
            EducationalLevel::MiddleSchool => EducationalLevel::MiddleSchool,
            EducationalLevel::HighSchool => EducationalLevel::HighSchool,
            EducationalLevel::Bachelor => EducationalLevel::Bachelor,
            EducationalLevel::Master => EducationalLevel::Master,
            EducationalLevel::Doctor => EducationalLevel::Doctor,
            EducationalLevel::Other => EducationalLevel::Other,
        }
    }
}

impl ToEntity<PersonIdNumberProvider> for PersonUsecaseSharedIdNumberProvider {
    fn to_entity(self) -> PersonIdNumberProvider {
        match self {
            PersonUsecaseSharedIdNumberProvider::NationalId => PersonIdNumberProvider::NationalId,
            PersonUsecaseSharedIdNumberProvider::Passport => PersonIdNumberProvider::Passport,
        }
    }
}

impl ToUsecaseOutput<CreateStudentUsecaseOutput> for PersonDbResponse {
    fn to_usecase_output(self) -> CreateStudentUsecaseOutput {
        CreateStudentUsecaseOutput {
            person_id: self.id.unwrap(), // todo()!
            student_id: None,            // todo()!
            polity_id: self.polity_id,
            polity_name: None,
            polity_location_name: None,
            polity_location_address: None,
            polity_location_email: None,
            saint_ids: self.saint_ids.clone(),
            christian_name: None,
            title: self.title.map(|t| t.parse().unwrap()),
            vow_progress: self.vow_progress.map(|t| t.parse().unwrap()),
            first_name: self.first_name.clone(),
            middle_name: self.middle_name.clone(),
            last_name: self.last_name.clone(),
            date_of_birth: self.date_of_birth,
            place_of_birth: self.place_of_birth.clone(),
            email: self.email.clone(),
            phone: self.phone.clone(),
            nationality: self.nationality.clone(),
            race: self.race.clone(),
            id_number: None,
            id_number_provider: None,
            date_of_issue: None,
            place_of_issue: None,
            address: self.address.clone(),
            // languages: None,  // todo()!
            // educational_stages: None, // todo()!
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
