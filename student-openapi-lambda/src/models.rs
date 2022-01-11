#![allow(unused_qualifications)]

use crate::models;
#[cfg(any(feature = "client", feature = "server"))]
use crate::header;

/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk_enum_derive::LabelledGenericEnum))]
pub enum EducationalLevel {
    #[serde(rename = "ELEMENTARY_SCHOOL")]
    ELEMENTARY_SCHOOL,
    #[serde(rename = "MIDDLE_SCHOOL")]
    MIDDLE_SCHOOL,
    #[serde(rename = "HIGH_SCHOOL")]
    HIGH_SCHOOL,
    #[serde(rename = "BACHELOR")]
    BACHELOR,
    #[serde(rename = "MASTER")]
    MASTER,
    #[serde(rename = "DOCTOR")]
    DOCTOR,
    #[serde(rename = "OTHER")]
    OTHER,
}

impl std::fmt::Display for EducationalLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            EducationalLevel::ELEMENTARY_SCHOOL => write!(f, "{}", "ELEMENTARY_SCHOOL"),
            EducationalLevel::MIDDLE_SCHOOL => write!(f, "{}", "MIDDLE_SCHOOL"),
            EducationalLevel::HIGH_SCHOOL => write!(f, "{}", "HIGH_SCHOOL"),
            EducationalLevel::BACHELOR => write!(f, "{}", "BACHELOR"),
            EducationalLevel::MASTER => write!(f, "{}", "MASTER"),
            EducationalLevel::DOCTOR => write!(f, "{}", "DOCTOR"),
            EducationalLevel::OTHER => write!(f, "{}", "OTHER"),
        }
    }
}

impl std::str::FromStr for EducationalLevel {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "ELEMENTARY_SCHOOL" => std::result::Result::Ok(EducationalLevel::ELEMENTARY_SCHOOL),
            "MIDDLE_SCHOOL" => std::result::Result::Ok(EducationalLevel::MIDDLE_SCHOOL),
            "HIGH_SCHOOL" => std::result::Result::Ok(EducationalLevel::HIGH_SCHOOL),
            "BACHELOR" => std::result::Result::Ok(EducationalLevel::BACHELOR),
            "MASTER" => std::result::Result::Ok(EducationalLevel::MASTER),
            "DOCTOR" => std::result::Result::Ok(EducationalLevel::DOCTOR),
            "OTHER" => std::result::Result::Ok(EducationalLevel::OTHER),
            _ => std::result::Result::Err(format!("Value not valid: {}", s)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct EducationalStage {
    #[serde(rename = "educationalLevel")]
    pub educational_level: models::EducationalLevel,

    #[serde(rename = "schoolName")]
    pub school_name: String,

    #[serde(rename = "major")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub major: Option<String>,

    #[serde(rename = "graduateYear")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub graduate_year: Option<f64>,

}

impl EducationalStage {
    pub fn new(educational_level: models::EducationalLevel, school_name: String, ) -> EducationalStage {
        EducationalStage {
            educational_level: educational_level,
            school_name: school_name,
            major: None,
            graduate_year: None,
        }
    }
}

/// Converts the EducationalStage value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for EducationalStage {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];
        // Skipping educationalLevel in query parameter serialization


        params.push("schoolName".to_string());
        params.push(self.school_name.to_string());


        if let Some(ref major) = self.major {
            params.push("major".to_string());
            params.push(major.to_string());
        }


        if let Some(ref graduate_year) = self.graduate_year {
            params.push("graduateYear".to_string());
            params.push(graduate_year.to_string());
        }

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a EducationalStage value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for EducationalStage {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub educational_level: Vec<models::EducationalLevel>,
            pub school_name: Vec<String>,
            pub major: Vec<String>,
            pub graduate_year: Vec<f64>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing EducationalStage".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "educationalLevel" => intermediate_rep.educational_level.push(<models::EducationalLevel as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "schoolName" => intermediate_rep.school_name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "major" => intermediate_rep.major.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "graduateYear" => intermediate_rep.graduate_year.push(<f64 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing EducationalStage".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(EducationalStage {
            educational_level: intermediate_rep.educational_level.into_iter().next().ok_or("educationalLevel missing in EducationalStage".to_string())?,
            school_name: intermediate_rep.school_name.into_iter().next().ok_or("schoolName missing in EducationalStage".to_string())?,
            major: intermediate_rep.major.into_iter().next(),
            graduate_year: intermediate_rep.graduate_year.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<EducationalStage> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<EducationalStage>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<EducationalStage>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for EducationalStage - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<EducationalStage> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <EducationalStage as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into EducationalStage - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ForeignLanguage {
    #[serde(rename = "language")]
    pub language: String,

    #[serde(rename = "level")]
    pub level: models::ForeignLanguageLevel,

}

impl ForeignLanguage {
    pub fn new(language: String, level: models::ForeignLanguageLevel, ) -> ForeignLanguage {
        ForeignLanguage {
            language: language,
            level: level,
        }
    }
}

/// Converts the ForeignLanguage value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ForeignLanguage {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        params.push("language".to_string());
        params.push(self.language.to_string());

        // Skipping level in query parameter serialization

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ForeignLanguage value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ForeignLanguage {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub language: Vec<String>,
            pub level: Vec<models::ForeignLanguageLevel>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ForeignLanguage".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "language" => intermediate_rep.language.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "level" => intermediate_rep.level.push(<models::ForeignLanguageLevel as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ForeignLanguage".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ForeignLanguage {
            language: intermediate_rep.language.into_iter().next().ok_or("language missing in ForeignLanguage".to_string())?,
            level: intermediate_rep.level.into_iter().next().ok_or("level missing in ForeignLanguage".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ForeignLanguage> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ForeignLanguage>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ForeignLanguage>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ForeignLanguage - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ForeignLanguage> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ForeignLanguage as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ForeignLanguage - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk_enum_derive::LabelledGenericEnum))]
pub enum ForeignLanguageLevel {
    #[serde(rename = "BEGINNER")]
    BEGINNER,
    #[serde(rename = "INTERMEDIATE")]
    INTERMEDIATE,
    #[serde(rename = "ADVANCED")]
    ADVANCED,
}

impl std::fmt::Display for ForeignLanguageLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ForeignLanguageLevel::BEGINNER => write!(f, "{}", "BEGINNER"),
            ForeignLanguageLevel::INTERMEDIATE => write!(f, "{}", "INTERMEDIATE"),
            ForeignLanguageLevel::ADVANCED => write!(f, "{}", "ADVANCED"),
        }
    }
}

impl std::str::FromStr for ForeignLanguageLevel {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "BEGINNER" => std::result::Result::Ok(ForeignLanguageLevel::BEGINNER),
            "INTERMEDIATE" => std::result::Result::Ok(ForeignLanguageLevel::INTERMEDIATE),
            "ADVANCED" => std::result::Result::Ok(ForeignLanguageLevel::ADVANCED),
            _ => std::result::Result::Err(format!("Value not valid: {}", s)),
        }
    }
}

/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk_enum_derive::LabelledGenericEnum))]
pub enum IdNumberProvider {
    #[serde(rename = "NATIONAL_ID")]
    NATIONAL_ID,
    #[serde(rename = "PASSPORT")]
    PASSPORT,
}

impl std::fmt::Display for IdNumberProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            IdNumberProvider::NATIONAL_ID => write!(f, "{}", "NATIONAL_ID"),
            IdNumberProvider::PASSPORT => write!(f, "{}", "PASSPORT"),
        }
    }
}

impl std::str::FromStr for IdNumberProvider {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "NATIONAL_ID" => std::result::Result::Ok(IdNumberProvider::NATIONAL_ID),
            "PASSPORT" => std::result::Result::Ok(IdNumberProvider::PASSPORT),
            _ => std::result::Result::Err(format!("Value not valid: {}", s)),
        }
    }
}

/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk_enum_derive::LabelledGenericEnum))]
pub enum StudentSortCriteria {
    #[serde(rename = "NAME_ASC")]
    NAME_ASC,
    #[serde(rename = "NAME_DESC")]
    NAME_DESC,
    #[serde(rename = "CHRISTIAN_NAME_ASC")]
    CHRISTIAN_NAME_ASC,
    #[serde(rename = "CHRISTIAN_NAME_DESC")]
    CHRISTIAN_NAME_DESC,
    #[serde(rename = "POLITY_NAME_ASC")]
    POLITY_NAME_ASC,
    #[serde(rename = "POLITY_NAME_DESC")]
    POLITY_NAME_DESC,
    #[serde(rename = "LOCATION_NAME_ASC")]
    LOCATION_NAME_ASC,
    #[serde(rename = "LOCATION_NAME_DESC")]
    LOCATION_NAME_DESC,
    #[serde(rename = "PLACE_OF_BIRTH_ASC")]
    PLACE_OF_BIRTH_ASC,
    #[serde(rename = "PLACE_OF_BIRTH_DESC")]
    PLACE_OF_BIRTH_DESC,
}

impl std::fmt::Display for StudentSortCriteria {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            StudentSortCriteria::NAME_ASC => write!(f, "{}", "NAME_ASC"),
            StudentSortCriteria::NAME_DESC => write!(f, "{}", "NAME_DESC"),
            StudentSortCriteria::CHRISTIAN_NAME_ASC => write!(f, "{}", "CHRISTIAN_NAME_ASC"),
            StudentSortCriteria::CHRISTIAN_NAME_DESC => write!(f, "{}", "CHRISTIAN_NAME_DESC"),
            StudentSortCriteria::POLITY_NAME_ASC => write!(f, "{}", "POLITY_NAME_ASC"),
            StudentSortCriteria::POLITY_NAME_DESC => write!(f, "{}", "POLITY_NAME_DESC"),
            StudentSortCriteria::LOCATION_NAME_ASC => write!(f, "{}", "LOCATION_NAME_ASC"),
            StudentSortCriteria::LOCATION_NAME_DESC => write!(f, "{}", "LOCATION_NAME_DESC"),
            StudentSortCriteria::PLACE_OF_BIRTH_ASC => write!(f, "{}", "PLACE_OF_BIRTH_ASC"),
            StudentSortCriteria::PLACE_OF_BIRTH_DESC => write!(f, "{}", "PLACE_OF_BIRTH_DESC"),
        }
    }
}

impl std::str::FromStr for StudentSortCriteria {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "NAME_ASC" => std::result::Result::Ok(StudentSortCriteria::NAME_ASC),
            "NAME_DESC" => std::result::Result::Ok(StudentSortCriteria::NAME_DESC),
            "CHRISTIAN_NAME_ASC" => std::result::Result::Ok(StudentSortCriteria::CHRISTIAN_NAME_ASC),
            "CHRISTIAN_NAME_DESC" => std::result::Result::Ok(StudentSortCriteria::CHRISTIAN_NAME_DESC),
            "POLITY_NAME_ASC" => std::result::Result::Ok(StudentSortCriteria::POLITY_NAME_ASC),
            "POLITY_NAME_DESC" => std::result::Result::Ok(StudentSortCriteria::POLITY_NAME_DESC),
            "LOCATION_NAME_ASC" => std::result::Result::Ok(StudentSortCriteria::LOCATION_NAME_ASC),
            "LOCATION_NAME_DESC" => std::result::Result::Ok(StudentSortCriteria::LOCATION_NAME_DESC),
            "PLACE_OF_BIRTH_ASC" => std::result::Result::Ok(StudentSortCriteria::PLACE_OF_BIRTH_ASC),
            "PLACE_OF_BIRTH_DESC" => std::result::Result::Ok(StudentSortCriteria::PLACE_OF_BIRTH_DESC),
            _ => std::result::Result::Err(format!("Value not valid: {}", s)),
        }
    }
}

/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk_enum_derive::LabelledGenericEnum))]
pub enum StudentTitle {
    #[serde(rename = "PRIEST")]
    PRIEST,
    #[serde(rename = "MONK")]
    MONK,
    #[serde(rename = "NUN")]
    NUN,
}

impl std::fmt::Display for StudentTitle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            StudentTitle::PRIEST => write!(f, "{}", "PRIEST"),
            StudentTitle::MONK => write!(f, "{}", "MONK"),
            StudentTitle::NUN => write!(f, "{}", "NUN"),
        }
    }
}

impl std::str::FromStr for StudentTitle {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "PRIEST" => std::result::Result::Ok(StudentTitle::PRIEST),
            "MONK" => std::result::Result::Ok(StudentTitle::MONK),
            "NUN" => std::result::Result::Ok(StudentTitle::NUN),
            _ => std::result::Result::Err(format!("Value not valid: {}", s)),
        }
    }
}

/// Student info
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
#[serde(rename = "student")]
pub struct StudentUpsert {
    #[serde(rename = "polityId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub polity_id: Option<uuid::Uuid>,

    #[serde(rename = "saintIdArray")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub saint_id_array: Option<Vec<uuid::Uuid>>,

    #[serde(rename = "title")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub title: Option<models::StudentTitle>,

    #[serde(rename = "progress")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub progress: Option<models::VowProgress>,

    #[serde(rename = "firstName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub first_name: Option<String>,

    #[serde(rename = "middleName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub middle_name: Option<String>,

    #[serde(rename = "lastName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_name: Option<String>,

    /// date of birth in format YYYY-MM-DD
    #[serde(rename = "dateOfBirth")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub date_of_birth: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "placeOfBirth")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub place_of_birth: Option<String>,

    #[serde(rename = "email")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub email: Option<String>,

    #[serde(rename = "phone")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub phone: Option<String>,

    #[serde(rename = "address")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub address: Option<String>,

    #[serde(rename = "nationality")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub nationality: Option<String>,

    #[serde(rename = "educationalStage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub educational_stage: Option<Vec<models::EducationalStage>>,

    #[serde(rename = "foreignLanguage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub foreign_language: Option<Vec<models::ForeignLanguage>>,

    #[serde(rename = "race")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub race: Option<String>,

    #[serde(rename = "idNumber")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id_number: Option<String>,

    #[serde(rename = "idNumberProvider")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id_number_provider: Option<models::IdNumberProvider>,

    #[serde(rename = "dateOfIssue")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub date_of_issue: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "placeOfIssue")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub place_of_issue: Option<String>,

}

impl StudentUpsert {
    pub fn new() -> StudentUpsert {
        StudentUpsert {
            polity_id: None,
            saint_id_array: None,
            title: None,
            progress: None,
            first_name: None,
            middle_name: None,
            last_name: None,
            date_of_birth: None,
            place_of_birth: None,
            email: None,
            phone: None,
            address: None,
            nationality: Some("Vietnamese".to_string()),
            educational_stage: None,
            foreign_language: None,
            race: None,
            id_number: None,
            id_number_provider: None,
            date_of_issue: None,
            place_of_issue: None,
        }
    }
}

/// Converts the StudentUpsert value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for StudentUpsert {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];
        // Skipping polityId in query parameter serialization

        // Skipping saintIdArray in query parameter serialization

        // Skipping title in query parameter serialization

        // Skipping progress in query parameter serialization


        if let Some(ref first_name) = self.first_name {
            params.push("firstName".to_string());
            params.push(first_name.to_string());
        }


        if let Some(ref middle_name) = self.middle_name {
            params.push("middleName".to_string());
            params.push(middle_name.to_string());
        }


        if let Some(ref last_name) = self.last_name {
            params.push("lastName".to_string());
            params.push(last_name.to_string());
        }

        // Skipping dateOfBirth in query parameter serialization


        if let Some(ref place_of_birth) = self.place_of_birth {
            params.push("placeOfBirth".to_string());
            params.push(place_of_birth.to_string());
        }


        if let Some(ref email) = self.email {
            params.push("email".to_string());
            params.push(email.to_string());
        }


        if let Some(ref phone) = self.phone {
            params.push("phone".to_string());
            params.push(phone.to_string());
        }


        if let Some(ref address) = self.address {
            params.push("address".to_string());
            params.push(address.to_string());
        }


        if let Some(ref nationality) = self.nationality {
            params.push("nationality".to_string());
            params.push(nationality.to_string());
        }

        // Skipping educationalStage in query parameter serialization

        // Skipping foreignLanguage in query parameter serialization


        if let Some(ref race) = self.race {
            params.push("race".to_string());
            params.push(race.to_string());
        }


        if let Some(ref id_number) = self.id_number {
            params.push("idNumber".to_string());
            params.push(id_number.to_string());
        }

        // Skipping idNumberProvider in query parameter serialization

        // Skipping dateOfIssue in query parameter serialization


        if let Some(ref place_of_issue) = self.place_of_issue {
            params.push("placeOfIssue".to_string());
            params.push(place_of_issue.to_string());
        }

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a StudentUpsert value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for StudentUpsert {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub polity_id: Vec<uuid::Uuid>,
            pub saint_id_array: Vec<Vec<uuid::Uuid>>,
            pub title: Vec<models::StudentTitle>,
            pub progress: Vec<models::VowProgress>,
            pub first_name: Vec<String>,
            pub middle_name: Vec<String>,
            pub last_name: Vec<String>,
            pub date_of_birth: Vec<chrono::DateTime::<chrono::Utc>>,
            pub place_of_birth: Vec<String>,
            pub email: Vec<String>,
            pub phone: Vec<String>,
            pub address: Vec<String>,
            pub nationality: Vec<String>,
            pub educational_stage: Vec<Vec<models::EducationalStage>>,
            pub foreign_language: Vec<Vec<models::ForeignLanguage>>,
            pub race: Vec<String>,
            pub id_number: Vec<String>,
            pub id_number_provider: Vec<models::IdNumberProvider>,
            pub date_of_issue: Vec<chrono::DateTime::<chrono::Utc>>,
            pub place_of_issue: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing StudentUpsert".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "polityId" => intermediate_rep.polity_id.push(<uuid::Uuid as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "saintIdArray" => return std::result::Result::Err("Parsing a container in this style is not supported in StudentUpsert".to_string()),
                    "title" => intermediate_rep.title.push(<models::StudentTitle as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "progress" => intermediate_rep.progress.push(<models::VowProgress as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "firstName" => intermediate_rep.first_name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "middleName" => intermediate_rep.middle_name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "lastName" => intermediate_rep.last_name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "dateOfBirth" => intermediate_rep.date_of_birth.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "placeOfBirth" => intermediate_rep.place_of_birth.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "email" => intermediate_rep.email.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "phone" => intermediate_rep.phone.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "address" => intermediate_rep.address.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "nationality" => intermediate_rep.nationality.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "educationalStage" => return std::result::Result::Err("Parsing a container in this style is not supported in StudentUpsert".to_string()),
                    "foreignLanguage" => return std::result::Result::Err("Parsing a container in this style is not supported in StudentUpsert".to_string()),
                    "race" => intermediate_rep.race.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "idNumber" => intermediate_rep.id_number.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "idNumberProvider" => intermediate_rep.id_number_provider.push(<models::IdNumberProvider as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "dateOfIssue" => intermediate_rep.date_of_issue.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "placeOfIssue" => intermediate_rep.place_of_issue.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing StudentUpsert".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(StudentUpsert {
            polity_id: intermediate_rep.polity_id.into_iter().next(),
            saint_id_array: intermediate_rep.saint_id_array.into_iter().next(),
            title: intermediate_rep.title.into_iter().next(),
            progress: intermediate_rep.progress.into_iter().next(),
            first_name: intermediate_rep.first_name.into_iter().next(),
            middle_name: intermediate_rep.middle_name.into_iter().next(),
            last_name: intermediate_rep.last_name.into_iter().next(),
            date_of_birth: intermediate_rep.date_of_birth.into_iter().next(),
            place_of_birth: intermediate_rep.place_of_birth.into_iter().next(),
            email: intermediate_rep.email.into_iter().next(),
            phone: intermediate_rep.phone.into_iter().next(),
            address: intermediate_rep.address.into_iter().next(),
            nationality: intermediate_rep.nationality.into_iter().next(),
            educational_stage: intermediate_rep.educational_stage.into_iter().next(),
            foreign_language: intermediate_rep.foreign_language.into_iter().next(),
            race: intermediate_rep.race.into_iter().next(),
            id_number: intermediate_rep.id_number.into_iter().next(),
            id_number_provider: intermediate_rep.id_number_provider.into_iter().next(),
            date_of_issue: intermediate_rep.date_of_issue.into_iter().next(),
            place_of_issue: intermediate_rep.place_of_issue.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<StudentUpsert> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<StudentUpsert>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<StudentUpsert>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for StudentUpsert - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<StudentUpsert> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <StudentUpsert as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into StudentUpsert - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// Student View
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
#[serde(rename = "student-view")]
pub struct StudentView {
    #[serde(rename = "id")]
    pub id: uuid::Uuid,

    #[serde(rename = "polityName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub polity_name: Option<String>,

    #[serde(rename = "polityLocationName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub polity_location_name: Option<String>,

    #[serde(rename = "polityLocationAddress")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub polity_location_address: Option<String>,

    #[serde(rename = "polityLocationEmail")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub polity_location_email: Option<String>,

    #[serde(rename = "christianName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub christian_name: Option<String>,

    #[serde(rename = "title")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub title: Option<models::StudentTitle>,

    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    /// date of birth in format YYYY-MM-DD
    #[serde(rename = "dateOfBirth")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub date_of_birth: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "placeOfBirth")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub place_of_birth: Option<String>,

    #[serde(rename = "email")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub email: Option<String>,

    #[serde(rename = "phone")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub phone: Option<String>,

    #[serde(rename = "undergraduateSchool")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub undergraduate_school: Option<String>,

}

impl StudentView {
    pub fn new(id: uuid::Uuid, ) -> StudentView {
        StudentView {
            id: id,
            polity_name: None,
            polity_location_name: None,
            polity_location_address: None,
            polity_location_email: None,
            christian_name: None,
            title: None,
            name: None,
            date_of_birth: None,
            place_of_birth: None,
            email: None,
            phone: None,
            undergraduate_school: None,
        }
    }
}

/// Converts the StudentView value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for StudentView {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];
        // Skipping id in query parameter serialization


        if let Some(ref polity_name) = self.polity_name {
            params.push("polityName".to_string());
            params.push(polity_name.to_string());
        }


        if let Some(ref polity_location_name) = self.polity_location_name {
            params.push("polityLocationName".to_string());
            params.push(polity_location_name.to_string());
        }


        if let Some(ref polity_location_address) = self.polity_location_address {
            params.push("polityLocationAddress".to_string());
            params.push(polity_location_address.to_string());
        }


        if let Some(ref polity_location_email) = self.polity_location_email {
            params.push("polityLocationEmail".to_string());
            params.push(polity_location_email.to_string());
        }


        if let Some(ref christian_name) = self.christian_name {
            params.push("christianName".to_string());
            params.push(christian_name.to_string());
        }

        // Skipping title in query parameter serialization


        if let Some(ref name) = self.name {
            params.push("name".to_string());
            params.push(name.to_string());
        }

        // Skipping dateOfBirth in query parameter serialization


        if let Some(ref place_of_birth) = self.place_of_birth {
            params.push("placeOfBirth".to_string());
            params.push(place_of_birth.to_string());
        }


        if let Some(ref email) = self.email {
            params.push("email".to_string());
            params.push(email.to_string());
        }


        if let Some(ref phone) = self.phone {
            params.push("phone".to_string());
            params.push(phone.to_string());
        }


        if let Some(ref undergraduate_school) = self.undergraduate_school {
            params.push("undergraduateSchool".to_string());
            params.push(undergraduate_school.to_string());
        }

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a StudentView value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for StudentView {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub id: Vec<uuid::Uuid>,
            pub polity_name: Vec<String>,
            pub polity_location_name: Vec<String>,
            pub polity_location_address: Vec<String>,
            pub polity_location_email: Vec<String>,
            pub christian_name: Vec<String>,
            pub title: Vec<models::StudentTitle>,
            pub name: Vec<String>,
            pub date_of_birth: Vec<chrono::DateTime::<chrono::Utc>>,
            pub place_of_birth: Vec<String>,
            pub email: Vec<String>,
            pub phone: Vec<String>,
            pub undergraduate_school: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing StudentView".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "id" => intermediate_rep.id.push(<uuid::Uuid as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "polityName" => intermediate_rep.polity_name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "polityLocationName" => intermediate_rep.polity_location_name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "polityLocationAddress" => intermediate_rep.polity_location_address.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "polityLocationEmail" => intermediate_rep.polity_location_email.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "christianName" => intermediate_rep.christian_name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "title" => intermediate_rep.title.push(<models::StudentTitle as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "name" => intermediate_rep.name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "dateOfBirth" => intermediate_rep.date_of_birth.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "placeOfBirth" => intermediate_rep.place_of_birth.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "email" => intermediate_rep.email.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "phone" => intermediate_rep.phone.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "undergraduateSchool" => intermediate_rep.undergraduate_school.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing StudentView".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(StudentView {
            id: intermediate_rep.id.into_iter().next().ok_or("id missing in StudentView".to_string())?,
            polity_name: intermediate_rep.polity_name.into_iter().next(),
            polity_location_name: intermediate_rep.polity_location_name.into_iter().next(),
            polity_location_address: intermediate_rep.polity_location_address.into_iter().next(),
            polity_location_email: intermediate_rep.polity_location_email.into_iter().next(),
            christian_name: intermediate_rep.christian_name.into_iter().next(),
            title: intermediate_rep.title.into_iter().next(),
            name: intermediate_rep.name.into_iter().next(),
            date_of_birth: intermediate_rep.date_of_birth.into_iter().next(),
            place_of_birth: intermediate_rep.place_of_birth.into_iter().next(),
            email: intermediate_rep.email.into_iter().next(),
            phone: intermediate_rep.phone.into_iter().next(),
            undergraduate_school: intermediate_rep.undergraduate_school.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<StudentView> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<StudentView>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<StudentView>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for StudentView - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<StudentView> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <StudentView as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into StudentView - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct StudentViewCollection {
    #[serde(rename = "students")]
    pub students: Vec<models::StudentView>,

    #[serde(rename = "hasMore")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub has_more: Option<bool>,

    #[serde(rename = "total")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub total: Option<i64>,

}

impl StudentViewCollection {
    pub fn new(students: Vec<models::StudentView>, ) -> StudentViewCollection {
        StudentViewCollection {
            students: students,
            has_more: None,
            total: None,
        }
    }
}

/// Converts the StudentViewCollection value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for StudentViewCollection {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];
        // Skipping students in query parameter serialization


        if let Some(ref has_more) = self.has_more {
            params.push("hasMore".to_string());
            params.push(has_more.to_string());
        }


        if let Some(ref total) = self.total {
            params.push("total".to_string());
            params.push(total.to_string());
        }

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a StudentViewCollection value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for StudentViewCollection {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub students: Vec<Vec<models::StudentView>>,
            pub has_more: Vec<bool>,
            pub total: Vec<i64>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing StudentViewCollection".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "students" => return std::result::Result::Err("Parsing a container in this style is not supported in StudentViewCollection".to_string()),
                    "hasMore" => intermediate_rep.has_more.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "total" => intermediate_rep.total.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing StudentViewCollection".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(StudentViewCollection {
            students: intermediate_rep.students.into_iter().next().ok_or("students missing in StudentViewCollection".to_string())?,
            has_more: intermediate_rep.has_more.into_iter().next(),
            total: intermediate_rep.total.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<StudentViewCollection> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<StudentViewCollection>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<StudentViewCollection>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for StudentViewCollection - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<StudentViewCollection> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <StudentViewCollection as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into StudentViewCollection - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk_enum_derive::LabelledGenericEnum))]
pub enum VowProgress {
    #[serde(rename = "SOLEMN_VOW")]
    SOLEMN_VOW,
    #[serde(rename = "SIMPLE_VOW")]
    SIMPLE_VOW,
    #[serde(rename = "NOVICE")]
    NOVICE,
    #[serde(rename = "PREPARATION")]
    PREPARATION,
}

impl std::fmt::Display for VowProgress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            VowProgress::SOLEMN_VOW => write!(f, "{}", "SOLEMN_VOW"),
            VowProgress::SIMPLE_VOW => write!(f, "{}", "SIMPLE_VOW"),
            VowProgress::NOVICE => write!(f, "{}", "NOVICE"),
            VowProgress::PREPARATION => write!(f, "{}", "PREPARATION"),
        }
    }
}

impl std::str::FromStr for VowProgress {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "SOLEMN_VOW" => std::result::Result::Ok(VowProgress::SOLEMN_VOW),
            "SIMPLE_VOW" => std::result::Result::Ok(VowProgress::SIMPLE_VOW),
            "NOVICE" => std::result::Result::Ok(VowProgress::NOVICE),
            "PREPARATION" => std::result::Result::Ok(VowProgress::PREPARATION),
            _ => std::result::Result::Err(format!("Value not valid: {}", s)),
        }
    }
}
