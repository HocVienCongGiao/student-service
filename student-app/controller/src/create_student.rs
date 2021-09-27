use crate::openapi::ToOpenApi;
use db_postgres::student_gateway::repository::StudentRepository;
use domain::usecases::create_student_usecase::{
    CreateStudentUsecase, CreateStudentUsecaseInput, CreateStudentUsecaseInteractor,
};
use domain::usecases::UsecaseError;
use hvcg_academics_openapi_student::models::Student as StudentOpenApi;

pub(crate) async fn from_openapi(student: &StudentOpenApi) -> Result<StudentOpenApi, UsecaseError> {
    // Init dependencies
    let client = db_postgres::connect().await;
    let student_repository = StudentRepository { client };

    // Inject dependencies to Interactor and invoke func
    let result = CreateStudentUsecaseInteractor::new(student_repository)
        .execute(CreateStudentUsecaseInput {
            id: None,
            polity_id: student.polity_id,
            saint_ids: student.saint_id_array.clone(),
            title: student.title.clone(),
            first_name: student.first_name.clone(),
            middle_name: student.middle_name.clone(),
            last_name: student.last_name.clone(),
            date_of_birth: student.date_of_birth,
            place_of_birth: student.place_of_birth.clone(),
            email: student.email.clone(),
            phone: student.phone.clone(),
            undergraduate_school: student.undergraduate_school.clone(),
        })
        .await;
    result.map(|res| res.to_openapi())
}