use super::models::{NewUser, User};
use super::schema::users::dsl::*;
use super::Pool;
use crate::diesel::QueryDsl;
use actix_web::{web, Error, HttpResponse};
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
    Ok(web::block(move || get_all_users(db))
        .await
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

pub async fn get_user_by_id(db: web::Data<Pool>, user_id: web::Path<i32>) -> Result<HttpResponse, Error> {
Ok(web::block(move || db_get_user_by_id(db, user_id.into_inner()))
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

pub async fn delete_user(db: web::Data<Pool>, user_id: web::Path<i32>) -> Result<HttpResponse, diesel::result::Error> {
    Ok(web::block(move || delete_single_user(db, user_id.into_inner()))
        .await
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

fn db_get_user_by_id(pool: web::Data<Pool>, user_id: i32) -> Result<User, diesel::result::Error> {
    let conn = pool.get().unwrap();
    users.find(user_id).get_result::<User>(&conn)
}


fn get_all_users(pool: web::Data<Pool>) -> Result<Vec<User>, diesel::result::Error> {
    let conn = pool.get().unwrap();
    let items = users.load::<User>(&conn)?;
    Ok(items)
}


fn add_single_user(db: web::Data<Pool>, item: web::Json<InputUser>) -> Result<User, diesel::result::Error>
{
    let conn = db.get().unwrap();
    let new_user = NewUser {
        first_name: &item.first_name,
        last_name: &item.last_name,
        email: &item.email,
        created_at: chrono::Local::now().naive_local(),
    };

    let res = insert_into(users).values(&new_user).get_result(&conn)?;

    Ok(res)
}

fn delete_single_user(db: web::Data<Pool>, user_id: i32) -> Result<usize, diesel::result::Error> {
    let conn = db.get().unwrap();

    let count = delete(users.find(user_id)).execute(&conn)?;

    Ok(count)
}







// pub async fn get_articles(db: web::Data<Pool>) -> Result<HttpResponse, diesel::result::Error> {
//     format!("hello from get articles")
// }


// // pub async fn save_articles

// // pub async fn delete_article