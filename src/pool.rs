use sqlx::postgres::PgConnectOptions;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

pub fn init_pool(db_connection: PgConnectOptions, timeout_in_millis: u64, pool_size: u32) -> PgPool {
    PgPoolOptions::new()
        .max_connections(pool_size)
        .idle_timeout(std::time::Duration::from_millis(timeout_in_millis))
        .connect_lazy_with(db_connection)
}