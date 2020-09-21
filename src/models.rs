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
pub struct NewUser {
    // For inserting new users
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub created_at: NaiveDateTime
}
