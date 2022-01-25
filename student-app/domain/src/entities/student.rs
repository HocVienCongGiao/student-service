use crate::entities::person::Person;
use chrono::NaiveDate;
use uuid::Uuid;

pub(crate) struct Student {
    pub student_id: Option<Uuid>,
    pub person: Option<Person>,
}

impl Student {
    pub(crate) fn is_valid(&self) -> bool {
        true
    }
}
