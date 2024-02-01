use crate::db::postgres;
use crate::model::user::User;
use actix_web::http::StatusCode;
use actix_web::{web, HttpResponse, ResponseError};
use anyhow::Context;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Deserialize, Debug)]
pub struct UserReq {
    id: i32,
}

#[derive(Serialize)]
struct UserResponse {
    id: i32,
    username: Option<String>,
}

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum ApiError {
    #[error("user not found")]
    NotFound,
    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),
}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match self {
            ApiError::NotFound => StatusCode::NOT_FOUND,
            ApiError::Unexpected(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

pub async fn find_user(
    pool: web::Data<PgPool>,
    query: web::Query<UserReq>,
) -> Result<HttpResponse, ApiError> {
    let req = query.into_inner();

    let user = postgres::find_user_by_id(req.id, &pool)
        .await
        .context("Failed to query user")?
        .ok_or(ApiError::NotFound)?;

    Ok(HttpResponse::Ok().json(UserResponse {
        id: user.id,
        username: user.username,
    }))
}

pub async fn insert_user(
    pool: web::Data<PgPool>,
    query: web::Query<User>,
) -> Result<HttpResponse, ApiError> {
    let user = query.into_inner();

    postgres::insert_user(&user, &pool)
        .await
        .context("Failed to insert user")?;

    Ok(HttpResponse::Ok().json("insert successful"))
}
