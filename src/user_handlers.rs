use super::models::{NewUser, User};
use super::schema::users::dsl::*;
use super::Pool;
use crate::diesel::QueryDsl;
use actix_web::{web, Error, HttpResponse};
use crate::diesel::RunQueryDsl;
use diesel::dsl::{delete, insert_into};
use serde::{Deserialize, Serialize};
use std::vec::Vec;
use bcrypt::{hash, verify, DEFAULT_COST};


// add crud
#[derive(Debug, Serialize, Deserialize)]

pub struct InputUser {
    pub user_id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}

// So to handle our auth we need to:
    // Hash the password
    // We can hash the password on creation, if we add it to the models/schema.
    // Once we do this, upon login, we can check if the hashed password,
    // and email match an email/hashed_pw pair in the database.

#[derive(Deserialize)]
pub struct RegisterUser {
    // Register user struct for representing a user *during* registration process
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub password: String,
    pub password_conf: String,
}
#[derive(Deserialize)]
pub struct AuthUser {
    // Auth user struct for representing a user *during* login process
    pub email: String,
    pub password: String
}

pub async fn get_users(db: web::Data<Pool>) -> Result<HttpResponse, Error> {
    // The web module provides helper functions and types for applications,
    // block executes a blocking function on a thread pool (the pool coming from our db)
    // until the function execution resolves.
    Ok(web::block(move || get_all_users(db))
        .await
        .map(|user| HttpResponse::Ok().json(user)) // Ok here corresponds to 200, with some syntactic sugar.
        .map_err(|_| HttpResponse::InternalServerError())?)
}

// pub async fn register_user() -> Result<HttpResponse, Error> {
//     Ok()
// }


pub async fn get_user_by_id(db: web::Data<Pool>, _id: web::Path<i32>) -> Result<HttpResponse, Error> {
    // We're returning result types, because we want to prepare for potential failure,
    // which requires a Error type, to be available along with the HttpResponse type that
    // will be returned if the request is successful.
Ok(web::block(move || db_get_user_by_id(db, _id.into_inner()))
    .await
    .map(|user| HttpResponse::Ok().json(user))
    .map_err(|_| HttpResponse::InternalServerError())?)
}

pub async fn add_user(db: web::Data<Pool>, item: web::Json<InputUser>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || add_single_user(db, item))
        .await
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

pub async fn delete_user(db: web::Data<Pool>, _id: web::Path<i32>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || delete_single_user(db, _id.into_inner()))
        .await
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

// pub fn register(new_user: web::Json<RegUser>)

fn db_get_user_by_id(pool: web::Data<Pool>, _id: i32) -> Result<User, diesel::result::Error> {
    let conn = pool.get().unwrap();
    users.find(_id).get_result::<User>(&conn)
}


fn get_all_users(pool: web::Data<Pool>) -> Result<Vec<User>, diesel::result::Error> {
    let conn = pool.get().unwrap();
    let items = users.load::<User>(&conn)?;
    Ok(items)
}


fn hash_pw(p_text: String) -> String {
    hash(p_text, DEFAULT_COST).unwrap()
}
// fn login(authed_u: web::Json<AuthUser>, pool: web::Data<Pool>, token: web::Data<CS>)
fn add_single_user(db: web::Data<Pool>, item: web::Json<InputUser>) -> Result<User, diesel::result::Error>
{
    let conn = db.get().unwrap();
    let hashed_pw = hash_pw(item.password.clone());
    let new_user = NewUser {
        user_id: &item.user_id,
        first_name: &item.first_name,
        last_name: &item.last_name,
        email: &item.email,
        password: &hashed_pw,
        created_at: chrono::Local::now().naive_local(),
    };

    let res = insert_into(users).values(&new_user).get_result(&conn)?;

    Ok(res)
}

fn delete_single_user(db: web::Data<Pool>, _id: i32) -> Result<usize, diesel::result::Error> {
    let conn = db.get().unwrap();

    let count = delete(users.find(_id)).execute(&conn)?;

    Ok(count)
}

// fn register_user(db: web::Data<Pool>, user: User) -> Result<usize, diesel::result::Error> {
    
// }







// pub async fn get_articles(db: web::Data<Pool>) -> Result<HttpResponse, diesel::result::Error> {
//     format!("hello from get articles")
// }


// // pub async fn save_articles

// // pub async fn delete_article