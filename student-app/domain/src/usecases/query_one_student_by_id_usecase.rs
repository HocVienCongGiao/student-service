use async_trait::async_trait;
use uuid::Uuid;

use crate::ports::polity_db_gateway::PolityDbGateway;
use crate::ports::saint_db_gateway::SaintDbGateway;
use crate::ports::student::models::student_dbresponse::Student as StudentDbResponse;
use crate::ports::student::student_db_gateway::StudentDbGateway;
use crate::usecases::student_usecase_shared_models::{
    QueryStudentUsecaseOutput, WithChristianName, WithPolity,
};
use crate::usecases::ToUsecaseOutput;

pub struct QueryOneStudentByIdUsecaseInteractor<
    A: StudentDbGateway,
    B: PolityDbGateway,
    C: SaintDbGateway,
> {
    student_db_gateway: A,
    polity_db_gateway: B,
    saint_db_gateway: C,
}

#[async_trait]
pub trait QueryOneStudentByIdUsecase {
    // InputBoundary
    async fn execute(&self, id: Uuid) -> Option<QueryStudentUsecaseOutput>;
}

#[async_trait]
impl<A, B, C> QueryOneStudentByIdUsecase for QueryOneStudentByIdUsecaseInteractor<A, B, C>
where
    A: StudentDbGateway + Sync + Send,
    B: PolityDbGateway + Sync + Send,
    C: SaintDbGateway + Sync + Send,
{
    async fn execute(&self, id: Uuid) -> Option<QueryStudentUsecaseOutput> {
        let usecase_output: Option<QueryStudentUsecaseOutput> = (*self)
            .student_db_gateway
            .find_one_by_id(id)
            .await
            .map(|response| response.to_usecase_output());

        return if let Some(usecase_output) = usecase_output {
            let mut usecase_output = usecase_output.with_polity(
                Some("1".to_string()),
                Some("1".to_string()),
                Some("1".to_string()),
                Some("1".to_string()),
            );
            if let Some(polity_id) = usecase_output.polity_id {
                if let Some(polity_db_response) =
                    (*self).polity_db_gateway.find_one_by_id(polity_id).await
                {
                    usecase_output = usecase_output.with_polity(
                        polity_db_response.name,
                        polity_db_response.location_name,
                        polity_db_response.location_address,
                        polity_db_response.location_email,
                    )
                }
            }
            let saint_ids = usecase_output.saint_ids.clone();
            if let Some(saint_ids) = saint_ids {
                for (_i, e) in saint_ids.iter().enumerate() {
                    if let Some(saint_db_response) =
                        (*self).saint_db_gateway.find_one_by_id(*e).await
                    {
                        usecase_output =
                            usecase_output.with_christian_name(saint_db_response.display_name)
                    }
                }
            }
            Some(usecase_output)
        } else {
            None
        };
    }
}

impl<A, B, C> QueryOneStudentByIdUsecaseInteractor<A, B, C>
where
    A: StudentDbGateway + Sync + Send,
    B: PolityDbGateway + Sync + Send,
    C: SaintDbGateway + Sync + Send,
{
    pub fn new(student_db_gateway: A, polity_db_gateway: B, saint_db_gateway: C) -> Self {
        QueryOneStudentByIdUsecaseInteractor {
            student_db_gateway,
            polity_db_gateway,
            saint_db_gateway,
        }
    }
}

impl ToUsecaseOutput<QueryStudentUsecaseOutput> for StudentDbResponse {
    fn to_usecase_output(self) -> QueryStudentUsecaseOutput {
        QueryStudentUsecaseOutput {
            id: self.id,
            polity_id: self.polity_id,
            polity_name: None,
            polity_location_name: None,
            polity_location_address: None,
            polity_location_email: None,
            saint_ids: self.saint_ids,
            christian_name: None,
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

impl WithPolity<QueryStudentUsecaseOutput> for QueryStudentUsecaseOutput {
    fn with_polity(
        mut self,
        name: Option<String>,
        location_name: Option<String>,
        location_address: Option<String>,
        location_email: Option<String>,
    ) -> QueryStudentUsecaseOutput {
        self.polity_name = name;
        self.polity_location_name = location_name;
        self.polity_location_address = location_address;
        self.polity_location_email = location_email;
        self
    }
}

impl WithChristianName<QueryStudentUsecaseOutput> for QueryStudentUsecaseOutput {
    fn with_christian_name(mut self, name: Option<String>) -> QueryStudentUsecaseOutput {
        if let Some(name) = name {
            let mut saint_names: Vec<String> = vec![];
            if self.christian_name.is_some() {
                saint_names = self.christian_name.unwrap();
            }
            saint_names.push(name);
            self.christian_name = Some(saint_names);
        }
        self
    }
}
