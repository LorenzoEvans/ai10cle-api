use super::models::{NewUser, User, Article, NewArticle};
use super::schema::articles::dsl::*;
use super::Pool;
use crate::diesel::QueryDsl;
use actix_web::{web, Error, HttpResponse};
use crate::diesel::RunQueryDsl;
use diesel::dsl::{delete, insert_into};
use serde::{Deserialize, Serialize};
use std::vec::Vec;


#[derive(Queryable, Deserialize, Serialize)]
pub struct InputArticle {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub link: String
}

pub async fn get_articles(db: web::Data<Pool>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || get_all_articles(db))
        .await
        .map(|article| HttpResponse::Ok().json(article))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

pub async fn get_article_by_id(db: web::Data<Pool>, _id: web::Path<i32>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || db_get_article_by_id(db, _id.into_inner()))
        .await
        .map(|article| HttpResponse::Ok().json(article))
        .map_err(|_| HttpResponse::InternalServerError())?)
}
pub async fn add_article(db: web::Data<Pool>, item: web::Json<InputArticle>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || add_single_article(db, item))
        .await
        .map(|article| HttpResponse::Ok().json(article))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

pub async fn delete_article(db: web::Data<Pool>, _id: web::Path<i32>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || delete_single_article(db, _id.into_inner()))
        .await
        .map(|article| HttpResponse::Ok().json(article))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

fn add_single_article(db: web::Data<Pool>, item: web::Json<InputArticle>) -> Result<Article, diesel::result::Error> {
    let conn = db.get().unwrap(); // Open up connection to Database
    let new_article = NewArticle {
        id: &item.id,
        user_id: &item.user_id,
        title: &item.title,
        link: &item.title
    };

    let res = insert_into(articles).values(&new_article).get_result(&conn)?;

    Ok(res)
}


fn db_get_article_by_id(pool: web::Data<Pool>, _id: i32) -> Result<Article, diesel::result::Error> {
    let conn = pool.get().unwrap(); // Open up connection to Database

    articles.find(_id).get_result::<Article>(&conn)
}

fn get_all_articles(pool: web::Data<Pool>) -> Result<Vec<Article>, diesel::result::Error> {
    let conn = pool.get().unwrap(); // Open up connection to Database

    let items = articles.load::<Article>(&conn)?;
    
    Ok(items)
}

fn delete_single_article(db: web::Data<Pool>, _id: i32) -> Result<usize, diesel::result::Error> {
    let conn = db.get().unwrap(); // Open up connection to Database

    let count = delete(articles.find(_id)).execute(&conn)?;

    Ok(count)
}