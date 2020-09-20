use crate::schema::*;
use serde::{Deserialize, Serialize};
use chrono::{NaiveDateTime};

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct User {
    // For querying existing users.
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub created_at: NaiveDateTime
}

#[derive(Insertable, Debug)]
#[table_name = "users"]
pub struct NewUser<'a> {
    // For inserting new users
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub email: &'a str,
    pub created_at: NaiveDateTime
}
