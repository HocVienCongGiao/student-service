use crate::ports::polity_db_gateway::PolityDbGateway;
use crate::ports::student_db_gateway::{StudentDbGateway, StudentDbResponse};
use crate::usecases::student_usecase_shared_models::{QueryStudentUsecaseOutput, WithPolity};
use crate::usecases::ToUsecaseOutput;
use async_trait::async_trait;
use chrono::Utc;
use uuid::Uuid;

pub struct QueryOneStudentByIdUsecaseInteractor<A: StudentDbGateway, B: PolityDbGateway> {
    student_db_gateway: A,
    polity_db_gateway: B,
}

#[async_trait]
pub trait QueryOneStudentByIdUsecase {
    // InputBoundary
    async fn execute(&self, id: Uuid) -> Option<QueryStudentUsecaseOutput>;
}

#[async_trait]
impl<A, B> QueryOneStudentByIdUsecase for QueryOneStudentByIdUsecaseInteractor<A, B>
where
    A: StudentDbGateway + Sync + Send,
    B: PolityDbGateway + Sync + Send,
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
            Some(usecase_output)
        } else {
            None
        };
    }
}

impl<A, B> QueryOneStudentByIdUsecaseInteractor<A, B>
where
    A: StudentDbGateway + Sync + Send,
    B: PolityDbGateway + Sync + Send,
{
    pub fn new(student_db_gateway: A, polity_db_gateway: B) -> Self {
        QueryOneStudentByIdUsecaseInteractor {
            student_db_gateway,
            polity_db_gateway,
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
