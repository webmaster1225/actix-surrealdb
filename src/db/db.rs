use once_cell::sync::Lazy;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;

pub static DB: Lazy<Surreal<Client>> = Lazy::new(|| Surreal::init());
