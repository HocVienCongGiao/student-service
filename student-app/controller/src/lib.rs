use hvcg_student_openapi_student::models::Student;

pub mod openapi;

pub async fn get_student_by_id() -> Student {
    Student {
        id: Option::from("53f549b9-99bf-4e12-88e3-c2f868953283".to_string()),
        polity_id: Option::from("4d084b56-54e1-4bd2-878e-c52675497c2b".to_string()),
        saint_id: Option::from("40e6215d-b5c6-4896-987c-f30f3678f608".to_string()),
        title: Option::from("PRIEST".to_string()),
        first_name: Option::from("Nguyen".to_string()),
        middle_name: Option::from("Huu".to_string()),
        last_name: Option::from("Chien".to_string()),
        date_of_birth: Option::from("2021-08-08".to_string()),
        place_of_birth: Option::from("Tra Vinh".to_string()),
        email: Option::from("binh@sunrise.vn".to_string()),
        phone: Option::from("84 1228019700".to_string()),
        school_name: Option::from("Dai Chung Vien Thanh Quy - Can Tho".to_string()),
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 99;
        assert_eq!(result, 99);
    }
}
