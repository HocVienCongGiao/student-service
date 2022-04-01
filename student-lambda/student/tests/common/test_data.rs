use hvcg_academics_openapi_student::models::{
    StudentUpsert, StudentView, StudentViewCollection,
};
use std::str::FromStr;
use uuid::Uuid;

pub fn prepare_student_view_openapi(student_id: Option<Uuid>) -> StudentView {
    StudentView {
        person_id: Some(Uuid::from_str("53f549b9-99bf-4e12-88e3-c2f868953283").unwrap()),
        student_id,
    }
}

pub fn prepare_student_upsert_openapi() -> StudentUpsert {
    StudentUpsert {
        person_id: Uuid::from_str("53f549b9-99bf-4e12-88e3-c2f868953283").unwrap(),
    }
}

pub fn prepare_student_view_collection_openapi() -> StudentViewCollection {
    StudentViewCollection {
        students: vec![prepare_student_view_openapi(None)],
        has_more: Some(false),
        total: Some(1),
    }
}
