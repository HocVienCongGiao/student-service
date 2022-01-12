use chrono::{Date, DateTime, NaiveDate};
use hvcg_academics_openapi_student::models::{
    EducationalLevel, EducationalStage, ForeignLanguage, ForeignLanguageLevel, IdNumberProvider,
    StudentTitle, StudentUpsert, StudentView, StudentViewCollection,
};
use std::str::FromStr;
use uuid::Uuid;

pub fn prepare_student_view_openapi(uuid: Option<Uuid>) -> StudentView {
    StudentView {
        id: uuid.unwrap_or_else(|| Uuid::from_str("53f549b9-99bf-4e12-88e3-c2f868953283").unwrap()),
        polity_name: Some("Cần Thơ".to_string()),
        polity_location_name: Some("Tòa Giám Mục Cần Thơ".to_string()),
        polity_location_address: Some("12 Nguyễn Trãi, Ninh Kiều, Cần Thơ".to_string()),
        polity_location_email: Some("binh@sunrise.vn".to_string()),
        christian_name: Some("Phêrô".to_string()),
        title: Some(StudentTitle::PRIEST),
        name: Some("Nguyễn Hữu Chiến".to_string()),
        date_of_birth: Some(NaiveDate::parse_from_str("1983-05-16", "%Y-%m-%d").unwrap()),
        place_of_birth: Some("Trà Vinh".to_string()),
        email: Some("binh@sunrise.vn".to_string()),
        phone: Some("+84 1228019700".to_string()),
        undergraduate_school: Some("Đại Chủng Viện Thánh Quý - Cần Thơ".to_string()),
    }
}

pub fn prepare_student_upsert_openapi() -> StudentUpsert {
    let date = NaiveDate::parse_from_str("1983-05-16", "%Y-%M-%d");
    match date {
        Ok(v) => println!("{:?}", v),
        Err(e) => println!("{:?}", e),
    }

    let student_educational_stage = EducationalStage {
        educational_level: EducationalLevel::HIGH_SCHOOL,
        school_name: "THPT Nguyễn Khuyến".to_string(),
        major: None,
        graduate_year: 2016.to_f64(),
    };
    let advanced_english = ForeignLanguage {
        language: "English".to_string(),
        level: ForeignLanguageLevel::ADVANCED,
    };
    let advanced_latin = ForeignLanguage {
        language: "latin".to_string(),
        level: ForeignLanguageLevel::ADVANCED,
    };
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
        date_of_birth: Some(NaiveDate::parse_from_str("1983-05-16", "%Y-%m-%d").unwrap()),
        place_of_birth: Some("Trà Vinh".to_string()),
        email: Some("binh@sunrise.vn".to_string()),
        phone: Some("+84 1228019700".to_string()),
        educational_stages: Some(vec![student_educational_stage]),
        nationality: Some("Vietnamese".to_string()),
        race: Some("Kinh".to_string()),
        id_number: Some("837837655558".to_string()),
        id_number_provider: Some(IdNumberProvider::NATIONAL_ID),
        date_of_issue: Some(NaiveDate::parse_from_str("2011-05-05", "%Y-%m-%d").unwrap()),
        place_of_issue: Some("TP.HCM".to_string()),
        address: Some("1000 CMT8 phường 3 quận Tân Bình, TP HCM".to_string()),
        foreign_language: Some(vec![advanced_english, advanced_latin]),
    }
}

pub fn prepare_student_view_collection_openapi() -> StudentViewCollection {
    StudentViewCollection {
        students: vec![prepare_student_view_openapi(None)],
        has_more: Some(false),
        total: Some(1),
    }
}
