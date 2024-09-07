use once_cell::sync::Lazy;
use serde::Serialize;
use serde_json::Value;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::{Error, Surreal};


pub static DB: Lazy<Surreal<Client>> = Lazy::new(|| Surreal::init());