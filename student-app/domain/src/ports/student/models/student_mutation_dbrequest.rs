use uuid::Uuid;

pub struct Student {
    pub student_id: Option<Uuid>,
    pub person_id: Option<Uuid>,
}