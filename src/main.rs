use rust_server::settings::get_config;
use rust_server::pool::init_pool;
use rust_server::server::run;
use std::net::TcpListener;
use std::time::Duration;

use redis_async_pool::{RedisConnectionManager, RedisPool, Ttl};

pub const CONFIG_LOCATION: &str = "config/setup.toml";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = get_config(CONFIG_LOCATION).expect("Failed to get settings");
    let db_pool = init_pool(config.db.with_db(), config.db.timeout_in_millis, config.db.pool_size);
    let kv_pool = RedisPool::new(
        RedisConnectionManager::new(redis::Client::open(config.kv.get_redis_url())?,
                                    true,
                                    Some(Ttl::Simple(Duration::from_millis(config.kv.timeout_in_millis)))),
        config.kv.pool_size,
    );
    let listener = TcpListener::bind(config.server.address_wo_protocol())?;

    run(listener, db_pool, kv_pool, config.server.concurrency_limit, config.server.client_timeout_in_millis)?.await?;
    Ok(())
}