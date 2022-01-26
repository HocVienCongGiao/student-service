use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use tokio_postgres::Row;
use uuid::Uuid;

pub fn get_uuid(row: &Row, col: &str) -> Uuid {
    row.get::<&str, Uuid>(col)
}

pub fn get_uuid_collection(row: &Row, col: &str) -> Vec<Uuid> {
    row.get::<&str, Vec<Uuid>>(col)
}

pub fn get_string(row: &Row, col: &str) -> String {
    row.get::<&str, String>(col)
}

// unused because we have fixed OpenAPIGen Date -> Rust Datetime
// pub fn get_datetime(row: &Row, col: &str) -> DateTime<Utc> {
//     row.get::<&str, DateTime<Utc>>(col)
// }

// pub fn get_datetime_from_db_date(row: &Row, col: &str) -> DateTime<Utc> {
//     let date = row.get::<&str, NaiveDate>(col);
//     let datetime = NaiveDateTime::new(date, NaiveTime::from_hms(0, 0, 0));
//
//     DateTime::from_utc(datetime, Utc)
// }

pub fn get_date(row: &Row, col: &str) -> NaiveDate {
    row.get::<&str, NaiveDate>(col)
}
