use chrono::DateTime;
use domain::boundaries::usecase_boundary::UsecaseError;
use hvcg_academics_openapi_student::models::{
    Student as StudentOpenApi, StudentView, StudentViewCollection,
};
use std::str::FromStr;
use uuid::Uuid;

pub mod openapi;

fn get_mock_student() -> StudentOpenApi {
    StudentOpenApi {
        id: Option::from(Uuid::from_str("53f549b9-99bf-4e12-88e3-c2f868953283").unwrap()),
        polity_id: Option::from(Uuid::from_str("53f549b9-99bf-4e12-88e3-c2f868953283").unwrap()),
        saint_id_array: Option::from(vec![
            Uuid::from_str("53f549b9-99bf-4e12-88e3-c2f868953283").unwrap()
        ]),
        title: Option::from("PRIEST".to_string()),
        first_name: Option::from("Nguyen".to_string()),
        middle_name: Option::from("Huu".to_string()),
        last_name: Option::from("Chien".to_string()),
        date_of_birth: Option::from(DateTime::from_str("1990-10-29 00:00:00+00:00").unwrap()),
        place_of_birth: Option::from("Tra Vinh".to_string()),
        email: Option::from("binh@sunrise.vn".to_string()),
        phone: Option::from("84 1228019700".to_string()),
        undergraduate_school: Option::from("Dai Chung Vien Thanh Quy - Can Tho".to_string()),
    }
}

pub async fn get_student_by_id(id: Uuid) -> Option<StudentOpenApi> {
    Option::from(get_mock_student())
}

pub async fn update_student(student_request: Option<StudentOpenApi>) -> Option<StudentOpenApi> {
    Option::from(get_mock_student())
}

pub async fn create_student(
    student_openapi: &StudentOpenApi,
) -> Result<StudentOpenApi, UsecaseError> {
    controller::create_student(student_openapi).await
}

pub async fn get_students(
    first_name: Option<String>,
    count: Option<u16>,
    offset: Option<u16>,
) -> StudentViewCollection {
    let mut result = StudentViewCollection {
        students: None,
        has_more: Option::from(false),
        total: Some(3),
    };

    let students = vec![
        StudentView {
            id: Option::from(Uuid::from_str("53f549b9-99bf-4e12-88e3-c2f868953283").unwrap()),
            polity_name: Option::from("Sai Gon".to_string()),
            polity_location_name: Option::from("Bean Institute".to_string()),
            polity_location_address: Option::from("372 CMT8 P10 Q3".to_string()),
            polity_location_email: Option::from("abc@gmail.com".to_string()),
            christian_name: Option::from("Peter".to_string()),
            title: Option::from("PRIEST".to_string()),
            name: Option::from("Nguyen Huu Chien".to_string()),
            date_of_birth: Option::from(DateTime::from_str("1990-10-29 00:00:00+00:00").unwrap()),
            place_of_birth: Option::from("Tra Vinh".to_string()),
            email: Option::from("binh@sunrise.vn".to_string()),
            phone: Option::from("84 1228019700".to_string()),
            undergraduate_school: Option::from("Dai Chung Vien Thanh Quy - Can Tho".to_string()),
            specialism: Option::from("TS Thanh Kinh - K3".to_string()),
        },
        StudentView {
            id: Option::from(Uuid::from_str("53f549b9-99bf-4e12-88e3-c2f868953283").unwrap()),
            polity_name: Option::from("Can Tho".to_string()),
            polity_location_name: Option::from("Bean Can Tho".to_string()),
            polity_location_address: Option::from("Can Tho address".to_string()),
            polity_location_email: Option::from("abc@cantho.com".to_string()),
            christian_name: Option::from("Peter".to_string()),
            title: Option::from("PRIEST".to_string()),
            name: Option::from("Nguyen Can Tho".to_string()),
            date_of_birth: Option::from(DateTime::from_str("1990-10-29 00:00:00+00:00").unwrap()),
            place_of_birth: Option::from("Tra Vinh".to_string()),
            email: Option::from("binh@sunrise.vn".to_string()),
            phone: Option::from("84 1228019700".to_string()),
            undergraduate_school: Option::from("Dai Chung Vien Thanh Quy - Can Tho".to_string()),
            specialism: Option::from("TS Thanh Kinh - K3".to_string()),
        },
    ];
    result.students.insert(students);
    result
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 99;
        assert_eq!(result, 99);
    }
}
