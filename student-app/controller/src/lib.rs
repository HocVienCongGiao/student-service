use hvcg_student_openapi_student::models::{Student, StudentCollection};
use uuid::Uuid;

pub mod openapi;

pub async fn get_student_by_id(id: Uuid) -> Option<Student> {
    Option::from(Student {
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
    })
}

pub async fn update_student(student_request: Option<Student>) -> Option<Student> {
    Option::from(Student {
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
    })
}

pub async fn create_student(student_request: Option<Student>) -> Option<Student> {
    Option::from(Student {
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
    })
}

pub async fn get_students(
    first_name: Option<String>,
    count: Option<u16>,
    offset: Option<u16>,
) -> StudentCollection {
    let mut result: StudentCollection = StudentCollection {
        students: None,
        has_more: None,
        total: None,
    };

    let students = vec![
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
        },
        Student {
            id: Option::from("c8c8a321-c18f-4c9f-aed4-02bf9a44580e".to_string()),
            polity_id: Option::from("7d796ee9-fa86-4621-9788-4899e9eb946f".to_string()),
            saint_id: Option::from("29159273-4b1b-4185-a21c-f008a8ad7ae4".to_string()),
            title: Option::from("PRIEST".to_string()),
            first_name: Option::from("Nguyen".to_string()),
            middle_name: Option::from("Huu".to_string()),
            last_name: Option::from("Tho".to_string()),
            date_of_birth: Option::from("2021-09-08".to_string()),
            place_of_birth: Option::from("Tra Vinh".to_string()),
            email: Option::from("binh@sunrise.vn".to_string()),
            phone: Option::from("84 1228019701".to_string()),
            school_name: Option::from("Dai Chung Vien Thanh Quy - Can Tho".to_string()),
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
