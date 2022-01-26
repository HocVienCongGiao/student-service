use chrono::NaiveDate;
use hvcg_academics_openapi_student::models::{
    IdNumberProvider, StudentTitle, StudentUpsert, StudentView, StudentViewCollection,
};
use std::str::FromStr;
use uuid::Uuid;

pub fn prepare_student_view_openapi(uuid: Option<Uuid>) -> StudentView {
    StudentView {
        id: uuid.unwrap_or_else(|| Uuid::from_str("ccb45678-69bb-4b54-9f09-3c8ab3c30999").unwrap()),
        polity_name: Some("Cần Thơ".to_string()),
        polity_location_name: Some("Tòa Giám Mục Cần Thơ".to_string()),
        polity_location_address: Some("12 Nguyễn Trãi, Ninh Kiều, Cần Thơ".to_string()),
        polity_location_email: Some("binh@sunrise.vn".to_string()),
        christian_name: Some("Phêrô".to_string()),
        title: Some(StudentTitle::PRIEST),
        name: Some("Nguyễn Hữu Chiến".to_string()),
        date_of_birth: Some(NaiveDate::from_str("1983-05-16").unwrap()),
        place_of_birth: Some("Trà Vinh".to_string()),
        email: Some("binh@sunrise.vn".to_string()),
        phone: Some("+84 1228019700".to_string()),
        // TODO: refactor this, no more undergrate_school
        undergraduate_school: None,
    }
}

pub fn prepare_student_upsert_openapi() -> StudentUpsert {
    StudentUpsert {
        polity_id: Some(Uuid::from_str("4d084b56-54e1-4bd2-878e-c52675497c2b").unwrap()),
        saint_id_array: Some(vec![
            Uuid::from_str("40e6215d-b5c6-4896-987c-f30f3678f608").unwrap()
        ]),
        title: Some(StudentTitle::PRIEST),
        progress: None,
        first_name: Some("Chiến".to_string()),
        middle_name: Some("Hữu".to_string()),
        last_name: Some("Nguyễn".to_string()),
        date_of_birth: Some(NaiveDate::from_str("1983-05-16").unwrap()),
        place_of_birth: Some("Trà Vinh".to_string()),
        email: Some("binh@sunrise.vn".to_string()),
        phone: Some("+84 1228019700".to_string()),
        nationality: Some("Vietnamese".to_string()),
        educational_stage: None,
        foreign_language: None,
        race: Some("Kinh".to_string()),
        id_number: Some("837837655557".to_string()),
        id_number_provider: Some(IdNumberProvider::NATIONAL_ID),
        date_of_issue: Some(NaiveDate::parse_from_str("2011-05-05", "%Y-%m-%d").unwrap()),
        place_of_issue: Some("TP.HCM".to_string()),
        address: Some("1000 CMT8 phường 3 quận Tân Bình, TP HCM".to_string()),
    }
}

pub fn prepare_student_view_collection_openapi() -> StudentViewCollection {
    StudentViewCollection {
        students: vec![prepare_student_view_openapi(None)],
        has_more: Some(false),
        total: Some(1),
    }
}
