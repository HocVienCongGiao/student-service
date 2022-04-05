use uuid::Uuid;

pub(crate) trait WithPolity<T> {
    fn with_polity(
        self,
        name: Option<String>,
        location_name: Option<String>,
        location_address: Option<String>,
        location_email: Option<String>,
    ) -> T;
}

pub(crate) trait WithChristianName<T> {
    fn with_christian_name(self, name: Option<String>) -> T;
}

pub(crate) trait WithStudentId<T> {
    fn with_student_id(self, student_id: Option<Uuid>) -> T;
}

#[derive(PartialEq, Clone)]
#[allow(non_camel_case_types)]
#[repr(C)]
pub enum StudentUsecaseSharedTitle {
    Priest,
    Monk,
    Nun,
}

impl std::str::FromStr for StudentUsecaseSharedTitle {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "PRIEST" => std::result::Result::Ok(StudentUsecaseSharedTitle::Priest),
            "MONK" => std::result::Result::Ok(StudentUsecaseSharedTitle::Monk),
            "NUN" => std::result::Result::Ok(StudentUsecaseSharedTitle::Nun),
            _ => std::result::Result::Err(format!("Value not valid: {}", s)),
        }
    }
}

#[derive(PartialEq, Clone)]
#[allow(non_camel_case_types)]
#[repr(C)]
pub enum StudentUsecaseSharedIdNumberProvider {
    NationalId,
    Passport,
}

impl std::str::FromStr for StudentUsecaseSharedIdNumberProvider {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "NATIONAL_ID" => {
                std::result::Result::Ok(StudentUsecaseSharedIdNumberProvider::NationalId)
            }
            "PASSPORT" => std::result::Result::Ok(StudentUsecaseSharedIdNumberProvider::Passport),
            _ => std::result::Result::Err(format!("Value not valid: {}", s)),
        }
    }
}

pub struct QueryStudentUsecaseOutput {
    pub student_id: Uuid,
    pub person_id: Uuid,
}
