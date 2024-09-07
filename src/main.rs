mod model;
mod routes;
mod db;

use actix_web::{ get, post, web, App, HttpResponse, HttpServer, Responder };
use once_cell::sync::Lazy;
use serde::{ Deserialize, Serialize };
use serde_json::Value;
use surrealdb::engine::remote::ws::{ Client, Ws };
use surrealdb::opt::auth::Root;
use surrealdb::{ Error, Surreal };
use db::db::DB;

#[derive(Debug, Serialize)]
struct Person {
    title: String,
    name: String,
    marketing: bool,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Connecting to the database...");
    DB.connect::<Ws>("127.0.0.1:8000").await.expect("Unnable to connect to the database");
    println!("Connected!");

    println!("Signing in to the database...");
    DB.signin(Root {
        username: "root",
        password: "root",
    }).await.expect("Unnable to sign in to the database");
    println!("Signed in!");

    println!("Setting up the namespace and database...");
    DB.use_ns("test").use_db("test").await.expect("Unnable to select namespace/database");
    println!("Setup complete!");

    println!("Starting Actix server on http://127.0.0.1:8081");
    HttpServer::new(move || {
        App::new()
            .service(index)
            .service(insert_person)
            .service(query_person)
            .service(routes::table::create_table)
    })
        .bind("127.0.0.1:8081")?
        .run().await
}
