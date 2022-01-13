use crate::Error;
use chrono::NaiveDate;
use controller::StudentCollectionQuery;
use domain::usecases::UsecaseError;
use hvcg_academics_openapi_student::models::{
    StudentSortCriteria, StudentUpsert, StudentViewCollection,
};
use jsonwebtoken::TokenData;
use lambda_http::http::header::{
    ACCESS_CONTROL_ALLOW_HEADERS, ACCESS_CONTROL_ALLOW_METHODS, ACCESS_CONTROL_ALLOW_ORIGIN,
    CONTENT_TYPE,
};
use lambda_http::http::{method, uri::Uri, HeaderValue};
use lambda_http::{handler, Body, Context, IntoResponse, Request, RequestExt, Response};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::env;
use std::str::FromStr;

fn from_query_param_to_string(request: &Request, param: &str) -> Option<String> {
    let query = request.query_string_parameters();
    query.get(param).map(|str| str.parse().unwrap())
}

pub fn from_request_to_id(req: &Request) -> Option<uuid::Uuid> {
    let path_parameters = req.path_parameters();
    let id_param = path_parameters.get("id");
    if let Some(id) = id_param {
        println!("id found");
        Some(uuid::Uuid::parse_str(id).unwrap())
    } else {
        println!("id not found");
        None
    }
}

pub fn from_request_to_collection_query(req: &Request) -> StudentCollectionQuery {
    let query = req.query_string_parameters();
    let param_date_of_birth = query.get("dateOfBirth");
    let param_date_of_birth = match param_date_of_birth {
        Some(param_date_of_birth) => {
            let param_date_of_birth = <NaiveDate as FromStr>::from_str(param_date_of_birth);
            match param_date_of_birth {
                Ok(param_date_of_birth) => Some(param_date_of_birth),
                Err(e) => {
                    println!("{:?}", e);
                    None
                }
            }
        }
        None => None,
    };

    let sort_criteria_dto: Vec<String> = query
        .get_all("sorts")
        .unwrap_or_default()
        .iter()
        .map(|e| e.to_string())
        .collect();

    let mut param_sorts: Option<Vec<StudentSortCriteria>> = None;

    if !sort_criteria_dto.is_empty() {
        let mut sort_criteria = Vec::new();
        sort_criteria_dto.iter().for_each(|criterion| {
            let s: StudentSortCriteria = criterion.parse().unwrap();
            sort_criteria.push(s);
        });
        param_sorts = Option::from(sort_criteria);
    }

    StudentCollectionQuery {
        name: from_query_param_to_string(req, "name"),
        email: from_query_param_to_string(req, "email"),
        phone: from_query_param_to_string(req, "phone"),
        undergraduate_school: from_query_param_to_string(req, "undergraduateSchool"),
        date_of_birth: param_date_of_birth,
        place_of_birth: from_query_param_to_string(req, "placeOfBirth"),
        polity_name: from_query_param_to_string(req, "polityName"),
        //specialism: from_query_param_to_string(req, "specialism"),
        sorts: param_sorts,
        offset: query.get("offset").map(|str| str.parse().unwrap()),
        count: query.get("count").map(|str| str.parse().unwrap()),
    }
}
