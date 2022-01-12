use hvcg_academics_openapi_student::models::{
    EducationalLevel, ForeignLanguageLevel, IdNumberProvider, StudentTitle,
    StudentUpsert as StudentUpsertOpenApi, StudentView as StudentViewOpenApi, VowProgress,
};

use db_postgres::polity_gateway::repository::PolityRepository;
use db_postgres::saint_gateway::repository::SaintRepository;
use db_postgres::student_gateway::repository::StudentRepository;
use domain::usecases::create_student_usecase::{
    CreateStudentUsecase, CreateStudentUsecaseInput, CreateStudentUsecaseInteractor,
};
use domain::usecases::person_usecase_shared_models::{
    PersonUsecaseSharedEducationalLevel, PersonUsecaseSharedEducationalStage,
    PersonUsecaseSharedIdNumberProvider, PersonUsecaseSharedLanguage,
    PersonUsecaseSharedLanguageLevel, PersonUsecaseSharedTitle, PersonUsecaseSharedVowProgress,
};
use domain::usecases::UsecaseError;

use crate::openapi::ToOpenApi;
use crate::ToUsecaseInput;

pub(crate) async fn from_openapi(
    student: StudentUpsertOpenApi,
) -> Result<StudentViewOpenApi, UsecaseError> {
    // Init dependencies
    let client = db_postgres::connect().await;
    let student_repository = StudentRepository { client };

    let polity_client = db_postgres::connect().await;
    let polity_repository = PolityRepository {
        client: polity_client,
    };

    let saint_client = db_postgres::connect().await;
    let saint_repository = SaintRepository {
        client: saint_client,
    };

    let person_client = db_postgres::connect().await;
    let person_repository = SaintRepository {
        client: person_client,
    };

    // Inject dependencies to Interactor and invoke func
    let result = CreateStudentUsecaseInteractor::new(
        student_repository,
        polity_repository,
        saint_repository,
        person_repository,
    )
    .execute(student.to_usecase_input())
    .await;
    result.map(|res| res.to_openapi())
}

impl ToUsecaseInput<CreateStudentUsecaseInput> for StudentUpsertOpenApi {
    fn to_usecase_input(self) -> CreateStudentUsecaseInput {
        let mut title: Option<PersonUsecaseSharedTitle> = None;
        let title_openapi = self.title;
        if let Some(title_openapi) = title_openapi {
            title = Some(match title_openapi {
                StudentTitle::MONK => PersonUsecaseSharedTitle::Monk,
                StudentTitle::NUN => PersonUsecaseSharedTitle::Nun,
                StudentTitle::PRIEST => PersonUsecaseSharedTitle::Priest,
            })
        }

        let mut vow_progress: Option<PersonUsecaseSharedVowProgress> = None;
        let vow_progress_openapi = self.progress;
        if let Some(vow_progress_openapi) = vow_progress_openapi {
            vow_progress = Some(match vow_progress_openapi {
                VowProgress::SOLEMN_VOW => PersonUsecaseSharedVowProgress::SolemnVow,
                VowProgress::SIMPLE_VOW => PersonUsecaseSharedVowProgress::SimpleVow,
                VowProgress::NOVICE => PersonUsecaseSharedVowProgress::Novice,
                VowProgress::PREPARATION => PersonUsecaseSharedVowProgress::Preparation,
            })
        }

        let mut educational_stages: Vec<PersonUsecaseSharedEducationalStage> = Vec::new();
        for stage in self.educational_stage.unwrap() {
            let mut educational_level: Option<PersonUsecaseSharedEducationalLevel> = None;
            let educational_level_openapi = stage.educational_level;
            if educational_level_openapi {
                educational_level = Some(match educational_level_openapi {
                    EducationalLevel::ELEMENTARY_SCHOOL => {
                        PersonUsecaseSharedEducationalLevel::ElementarySchool
                    }
                    EducationalLevel::MIDDLE_SCHOOL => {
                        PersonUsecaseSharedEducationalLevel::MiddleSchool
                    }
                    EducationalLevel::HIGH_SCHOOL => {
                        PersonUsecaseSharedEducationalLevel::HighSchool
                    }
                    EducationalLevel::BACHELOR => PersonUsecaseSharedEducationalLevel::Bachelor,
                    EducationalLevel::MASTER => PersonUsecaseSharedEducationalLevel::Master,
                    EducationalLevel::DOCTOR => PersonUsecaseSharedEducationalLevel::Doctor,
                    EducationalLevel::OTHER => PersonUsecaseSharedEducationalLevel::Other,
                })
            }
            educational_stages.push(PersonUsecaseSharedEducationalStage {
                educational_level,
                school_name: Some(stage.school_name),
                major: stage.major,
                graduate_year: stage.graduate_year,
            })
        }

        let mut languages: Vec<PersonUsecaseSharedLanguage> = Vec::new();
        for language in self.foreign_language.unwrap() {
            let mut language_level: Option<PersonUsecaseSharedLanguageLevel> = None;
            let language_level_openapi = language.level;
            if let Some(language_level_openapi) = language_level_openapi {
                language_level = Some(match language_level_openapi {
                    ForeignLanguageLevel::BEGINNER => PersonUsecaseSharedLanguageLevel::Beginner,
                    ForeignLanguageLevel::INTERMEDIATE => {
                        PersonUsecaseSharedLanguageLevel::Intermediate
                    }
                    ForeignLanguageLevel::ADVANCED => PersonUsecaseSharedLanguageLevel::Advanced,
                })
            }
            languages.push(PersonUsecaseSharedLanguage {
                language: language.language,
                level: language_level.unwrap(),
            });
        }

        let mut id_number_provider: Option<PersonUsecaseSharedIdNumberProvider> = None;
        let id_number_provider_openapi = self.id_number_provider;
        if let Some(id_number_provider_openapi) = id_number_provider_openapi {
            id_number_provider = Some(match id_number_provider_openapi {
                IdNumberProvider::NATIONAL_ID => PersonUsecaseSharedIdNumberProvider::NationalId,
                IdNumberProvider::PASSPORT => PersonUsecaseSharedIdNumberProvider::Passport,
            })
        }

        CreateStudentUsecaseInput {
            polity_id: self.polity_id,
            saint_ids: self.saint_id_array.clone(),
            title,
            vow_progress,
            first_name: self.first_name.clone(),
            middle_name: self.middle_name.clone(),
            last_name: self.last_name.clone(),
            date_of_birth: self.date_of_birth,
            place_of_birth: self.place_of_birth.clone(),
            email: self.email.clone(),
            phone: self.phone.clone(),
            educational_stages: Some(educational_stages),
            nationality: self.nationality,
            race: self.race,
            id_number: self.id_number,
            id_number_provider,
            date_of_issue: self.date_of_issue,
            place_of_issue: self.place_of_issue,
            address: self.address,
            languages: Some(languages),
        }
    }
}
