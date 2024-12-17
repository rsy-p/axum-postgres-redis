use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use bb8_redis::RedisConnectionManager;
use tokio_postgres::NoTls;

pub type DBPool = Pool<PostgresConnectionManager<NoTls>>;

pub type RedisPool = Pool<RedisConnectionManager>;

pub struct AppState {
    pub config: Config,
    pub db: DBPool,
    pub redis: RedisPool,
}

#[derive(clap::Parser, Debug)]
pub struct Config {
    #[clap(long, env, default_value = "8080")]
    pub server_port: u32,

    #[clap(long, env, default_value = "/")]
    pub context_path: String,

    #[clap(long, env, default_value = "/")]
    pub public_path: String,

    #[clap(long, env)]
    pub jwt_secret: String,

    #[clap(long, env)]
    pub database_url: String,

    #[clap(long, env, default_value = "10")]
    pub max_db_poll_size: u32,

    #[clap(long, env)]
    pub redis_url: String,

    #[clap(long, env, default_value = "10")]
    pub max_redis_poll_size: u32,
}
