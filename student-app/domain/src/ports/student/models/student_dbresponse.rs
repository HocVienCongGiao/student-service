use chrono::NaiveDate;
use uuid::Uuid;

pub struct StudentInsert {
    pub person_id: Uuid,
    pub student_id: Uuid,
}

pub struct Student {
    pub id: Uuid,
    pub polity_id: Option<uuid::Uuid>,
    pub saint_ids: Option<Vec<uuid::Uuid>>,
    pub title: Option<String>,
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub last_name: Option<String>,
    pub date_of_birth: Option<NaiveDate>,
    pub place_of_birth: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub undergraduate_school: Option<String>,
}

pub struct StudentCollection {
    pub collection: Vec<Student>,
    pub has_more: Option<bool>,
    pub total: i64,
}
