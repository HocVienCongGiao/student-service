use uuid::Uuid;

pub(crate) struct Student {
    pub id: Option<Uuid>,
    pub person_id: Option<Uuid>,
}

impl Student {
    pub(crate) fn is_valid(&self) -> bool {
        true
    }
}
