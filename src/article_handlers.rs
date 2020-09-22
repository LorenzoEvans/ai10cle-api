use super::models::{NewUser, User, Article, NewArticle};
use super::schema::articles::dsl::*;
use super::Pool;
use crate::diesel::QueryDsl;
use actix_web::{web, Error, HttpResponse};
use crate::diesel::RunQueryDsl;
use diesel::dsl::{delete, insert_into};
use serde::{Deserialize, Serialize};
use std::vec::Vec;


#[derive(Queryable)]
pub struct InputArticle {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub link: String
}

pub async fn add_article(db: web::Data<Pool>, item: web::Json<InputArticle>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || add_single_article(db, item))
        .await
        .map(|article| HttpResponse::Ok().json(article))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

fn add_single_article(db: web::Data<Pool>, item: web::Json<InputArticle>) -> Result<Article, diesel::result::Error> {
    let conn = db.get().unwrap();
    let new_article = NewArticle {
        id: &item.id,
        user_id: &item.user_id,
        title: &item.title,
        link: &item.title
    };

    let res = insert_into(articles).values(&new_article).get_result(&conn)?;

    Ok(res)
}