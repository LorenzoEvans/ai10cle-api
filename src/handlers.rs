use actix_web::{web, Error, HttpResponse};
use super::models::{NewUser, User};
use super::schema::users::dsl::*;
use super::Pool;
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use diesel::dsl::{delete, insert_into};
use serde::{Deserialize, Serialize};
use std::vec::Vec;


#[derive(Debug, Serialize, Deserialize)]

pub struct InputUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}
pub async fn get_users(db: web::Data<Pool>) -> Result<HttpResponse, Error> {
    format!("hello from get users")
}

pub async fn get_user_by_id(db: web::Data<Pool>) -> Result<HttpResponse, Error> {
    format!("hello from get users by id")
}

pub async fn add_user(db: web::Data<Pool>) -> Result<HttpResponse, Error> {
    format!("hello from add user")
}

pub async fn delete_user(db: web::Data<Pool>) -> Result<HttpResponse, Error> {
    format!("hello from delete user")
}

pub async fn get_articles(db: web::Data<Pool>) -> Result<HttpResponse, Error> {
    format!("hello from get articles")
}

// pub async fn save_articles

// pub async fn delete_article