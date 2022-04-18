use actix_web::{web, Error, HttpResponse};
use diesel::dsl::insert_into;
use diesel::{QueryDsl, RunQueryDsl};
use serde::{Deserialize, Serialize};

use super::super::schema::itg_user::dsl::*;
use crate::{
    models::user::{CreateUser, User},
    Pool,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub first_name: String,
    pub last_name: String,
}

pub async fn get_all(db: web::Data<Pool>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || db_get_all(db))
        .await
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

fn db_get_all(pool: web::Data<Pool>) -> Result<Vec<User>, diesel::result::Error> {
    let conn = pool.get().unwrap();
    let items = itg_user.load::<User>(&conn)?;
    Ok(items)
}

pub async fn get_by_id(
    db: web::Data<Pool>,
    user_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    Ok(web::block(move || db_get_by_id(db, user_id.into_inner()))
        .await
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

fn db_get_by_id(pool: web::Data<Pool>, user_id: i32) -> Result<User, diesel::result::Error> {
    let conn = pool.get().unwrap();
    itg_user.find(user_id).get_result::<User>(&conn)
}

pub async fn create(
    db: web::Data<Pool>,
    item: web::Json<CreateUserRequest>,
) -> Result<HttpResponse, Error> {
    Ok(web::block(move || db_create(db, item))
        .await
        .map(|user| HttpResponse::Created().json(user))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

fn db_create(
    db: web::Data<Pool>,
    item: web::Json<CreateUserRequest>,
) -> Result<User, diesel::result::Error> {
    let conn = db.get().unwrap();
    let new_user = CreateUser {
        first_name: &item.first_name,
        last_name: &item.last_name,
    };

    let res = insert_into(itg_user).values(&new_user).get_result(&conn)?;
    Ok(res)
}

pub async fn delete(db: web::Data<Pool>, user_id: web::Path<i32>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || db_delete(db, user_id.into_inner()))
        .await
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

fn db_delete(db: web::Data<Pool>, user_id: i32) -> Result<usize, diesel::result::Error> {
    let conn = db.get().unwrap();
    let count = diesel::dsl::delete(itg_user.find(user_id)).execute(&conn)?;
    Ok(count)
}
