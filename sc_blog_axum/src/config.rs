use dotenv::dotenv;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Web {
    pub addr: String,
}

#[derive(Deserialize)]
pub struct Db {
    pub max_connections: u32,
    pub url: String,
}
#[derive(Deserialize)]
pub struct Config {
    pub web: Web,
    pub db: Db,
}
// impl Config {
// pub fn from_env() -> Result<Self, anyhow::Error> {}
// }
