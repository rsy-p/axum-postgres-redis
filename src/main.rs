use axum_postgres_redis::{AppState, Config, DBPool, RedisPool};
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use bb8_redis::RedisConnectionManager;
use clap::Parser;
use dotenv::dotenv;
use tokio_postgres::NoTls;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{fmt, EnvFilter};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let state = init().await?;
    axum_postgres_redis::server(state).await?;
    Ok(())
}

async fn init() -> anyhow::Result<AppState> {
    init_tracing();
    let config = Config::parse();
    let db = init_db(&config).await?;
    let redis = init_redis(&config).await?;
    Ok(AppState { db, config, redis })
}

fn init_tracing() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();
}

async fn init_db(config: &Config) -> anyhow::Result<DBPool> {
    let manager = PostgresConnectionManager::new_from_stringlike(&config.database_url, NoTls)?;
    let pool = Pool::builder()
        .max_size(config.max_db_poll_size)
        .build(manager)
        .await?;
    Ok(pool)
}

async fn init_redis(config: &Config) -> anyhow::Result<RedisPool> {
    let manager = RedisConnectionManager::new(&*config.redis_url)?;
    let pool = Pool::builder()
        .max_size(config.max_redis_poll_size)
        .build(manager)
        .await?;
    Ok(pool)
}
