use crate::schema::{articles, users};
use serde::{Deserialize, Serialize};
use chrono::{NaiveDateTime};
use diesel;

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct User {
    // For querying existing users.
    pub user_id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime
}

// impl User
// https://dev.to/werner/practical-rust-web-development-authentication-3ppg
// pub struct Article
#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Article {
    pub user_id: i32,
    pub id: i32,
    pub title: String,
    pub link: String,
}

#[derive(Insertable, Debug, Queryable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    // For inserting new users
    pub user_id: &'a i32,
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub email: &'a str,
    pub password: &'a str,
    pub created_at: NaiveDateTime
}

#[derive(Insertable, Debug, Identifiable, Associations, Queryable)]
#[belongs_to(User)]
#[table_name = "articles"]
pub struct NewArticle<'a> {
    pub user_id: &'a i32,
    pub id: &'a i32,
    pub title: &'a str,
    pub link: &'a str
}