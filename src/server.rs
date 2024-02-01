use crate::routes::users::{find_user, insert_user};
use crate::routes::store::{kv_get, kv_set};
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;
use std::time::Duration;
use actix_web::web::Data;
use redis_async_pool::RedisPool;
use tracing_actix_web::TracingLogger;

pub fn run(listener: TcpListener, db_pool: PgPool,
           kv_pool: RedisPool, concurrency_limit: usize, client_timeout_in_millis: u64) -> Result<Server, anyhow::Error> {
    let db_pool = Data::new(db_pool);
    let kv_pool = Data::new(kv_pool);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/user", web::get().to(find_user))
            .route("/user", web::post().to(insert_user))
            .route("/store", web::get().to(kv_get))
            .route("/store", web::post().to(kv_set))
            .app_data(db_pool.clone())
            .app_data(kv_pool.clone())
    })
        .max_connection_rate(concurrency_limit)
        .client_disconnect_timeout(Duration::from_millis(client_timeout_in_millis))
        .listen(listener)?
        .run();

    Ok(server)
}