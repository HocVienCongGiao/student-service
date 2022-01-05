use chrono::{DateTime, NaiveDate, Utc};
use uuid::Uuid;

pub(crate) struct Student {
    pub id: Option<Uuid>,
    pub person_id: Option<Uuid>,
    pub date_of_birth: Option<NaiveDate>,
    pub place_of_birth: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub educational_stage: Option<Vec<EducationalStage>>,
    pub nationality: Option<String>,
    pub race: Option<String>,
    pub address: Option<String>,
}

impl Student {
    pub(crate) fn is_valid(&self) -> bool {
        true
    }
}
