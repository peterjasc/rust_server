use actix_web::{HttpResponse, ResponseError, web};
use actix_web::http::StatusCode;
use redis::{RedisError};
use redis_async_pool::RedisPool;
use redis::AsyncCommands;
use redis_async_pool::deadpool::managed::PoolError;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct KVGetReq {
    key: String,
}

#[derive(Deserialize, Debug)]
pub struct KVSetReq {
    key: String,
    val: String,
}

#[derive(thiserror::Error, Debug)]
pub enum KeyValueError {
    #[error(transparent)]
    Redis(#[from] PoolError<RedisError>),
    #[error(transparent)]
    RedisPool(#[from] RedisError),
}

impl ResponseError for KeyValueError {
    fn status_code(&self) -> StatusCode {
        match self {
            KeyValueError::Redis(_) => StatusCode::INTERNAL_SERVER_ERROR,
            KeyValueError::RedisPool(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

pub async fn kv_get(kv_pool: web::Data<RedisPool>, query: web::Query<KVGetReq>) -> Result<HttpResponse, KeyValueError> {
    let kv_req = query.into_inner();
    let mut connection = kv_pool.get().await?;

    let value: Vec<u8> = connection.get(kv_req.key.as_bytes()).await?;

    let res = String::from_utf8_lossy(&value);

    Ok(HttpResponse::Ok().json(res))
}

pub async fn kv_set(kv_pool: web::Data<RedisPool>, query: web::Query<KVSetReq>) -> Result<HttpResponse, KeyValueError> {
    let kv_req = query.into_inner();
    let mut connection = kv_pool.get().await?;

    connection.set(kv_req.key.as_bytes(), kv_req.val.as_bytes()).await?;
    Ok(HttpResponse::Ok().json("set successful"))
}