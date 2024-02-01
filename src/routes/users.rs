use std::fmt::Debug;
use actix_web::{web, HttpResponse, ResponseError};
use actix_web::http::StatusCode;
use anyhow::Context;
use serde::{Serialize, Deserialize};
use sqlx::PgPool;
use crate::db::postgres;
use crate::model::user::User;

#[derive(Deserialize, Debug)]
pub struct UserReq {
    id: i32,
}

#[derive(Serialize)]
struct UserResponse {
    id: i32,
    username: Option<String>,
}

#[derive(Debug,thiserror::Error)]
pub enum DatabaseError {
    #[error("{0}")]
    RequestError(actix_web::Error),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl ResponseError for DatabaseError {
    fn status_code(&self) -> StatusCode {
        match self {
            DatabaseError::RequestError(_) => StatusCode::BAD_REQUEST,
            DatabaseError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

pub async fn find_user(pool: web::Data<PgPool>, query: web::Query<UserReq>) -> Result<HttpResponse, actix_web::Error> {
    let user = query.into_inner();
    let res = postgres::find_user_by_id(user.id, &pool)
        .await;

    let user_resp = match res {
        Ok(user) => UserResponse {
            id: user.id,
            username: user.username,
        },
        Err(e) => return Err(actix_web::error::ErrorInternalServerError(e))
    };

    Ok(HttpResponse::Ok().json(user_resp))
}

pub async fn insert_user(pool: web::Data<PgPool>, query: web::Query<User>) -> Result<HttpResponse, DatabaseError> {
    let transaction = pool
        .begin()
        .await
        .context("Failed to get pool")?;

    let user = query.into_inner();

    let _ = postgres::insert_user(&user,  transaction)
        .await
        .map_err(|e| DatabaseError::RequestError(actix_web::error::ErrorInternalServerError(e)))?;

    Ok(HttpResponse::Ok().json("insert successful"))
}
