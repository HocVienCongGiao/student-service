use crate::SortDirection;
use chrono::NaiveDate;
use uuid::Uuid;

pub struct StudentQuery {
    pub id: Option<Uuid>,
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub undergraduate_school: Option<String>,
    pub date_of_birth: Option<NaiveDate>,
    pub place_of_birth: Option<String>,
    pub polity_name: Option<String>,
    //pub specialism: Option<String>,
    pub sort_request: Option<StudentSort>,
    pub offset: Option<i64>,
    pub count: Option<i64>,
}

pub struct StudentSort {
    pub sort_criteria: Vec<StudentSortCriteria>,
}

pub struct StudentSortCriteria {
    pub field: StudentSortField,
    pub direction: SortDirection,
}
#[derive(strum_macros::Display)]
pub enum StudentSortField {
    FirstName,
    MiddleName,
    LastName,
    ChristianName,
    PolityName,
    LocationName,
    PlaceOfBirth,
}
