use log::info;

use tokio_postgres::NoTls;
use dotenv::dotenv;
use deadpool_postgres::Pool;


mod config {
    pub use ::config::ConfigError;
    use serde::Deserialize;
    #[derive(Deserialize)]
    pub struct Config {
        pub server_addr: String,
        pub pg: deadpool_postgres::Config,
    }
    impl Config {
        pub fn from_env() -> Result<Self, ConfigError> {
            let mut cfg = ::config::Config::new();
            cfg.merge(::config::Environment::new())?;
            cfg.try_into()
        }
    }
}


pub fn get_client_pool() -> Pool {
    dotenv().ok();

    let config = config::Config::from_env().unwrap();

    config.pg.create_pool(NoTls).unwrap()

}


