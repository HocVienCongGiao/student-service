use chrono::DateTime;
use hvcg_academics_openapi_student::models::StudentView;
use std::str::FromStr;
use uuid::Uuid;

pub fn prepare_student_view_openapi() -> StudentView {
    StudentView {
        id: Uuid::from_str("53f549b9-99bf-4e12-88e3-c2f868953283").unwrap(),
        polity_name: Some("Cần Thơ".to_string()),
        polity_location_name: Some("Tòa Giám Mục Cần Thơ".to_string()),
        polity_location_address: Some("12 Nguyễn Trãi, Ninh Kiều, Cần Thơ".to_string()),
        polity_location_email: Some("binh@sunrise.vn".to_string()),
        christian_name: Some("Phêrô".to_string()),
        title: Some("PRIEST".to_string()),
        name: Some("Nguyễn Hữu Chiến".to_string()),
        date_of_birth: Some(DateTime::from_str("1983-05-16 00:00:00+00:00").unwrap()),
        place_of_birth: Some("Tra Vinh".to_string()),
        email: Some("binh@sunrise.vn".to_string()),
        phone: Some("+84 1228019700".to_string()),
        undergraduate_school: Some("Đại Chủng Viện Thánh Quý - Cần Thơ".to_string()),
    }
}
