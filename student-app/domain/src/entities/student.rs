use uuid::Uuid;

pub struct Student {
    pub student_id: Option<Uuid>,
    pub person_id: Option<Uuid>,
}

impl Student {
    pub(crate) fn is_valid(&self) -> bool {
        true
    }
}
