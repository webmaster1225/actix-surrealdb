mod model;
mod routes;
mod db;

use actix_web::{ App, HttpServer };
use surrealdb::opt::auth::Root;
use surrealdb::engine::remote::ws::Ws;
use db::db::DB;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Connecting to the database...");
    // create_surrealdb("AirTable.surql", "ws://localhost:8000", "my_namespace", "my_database").await;
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
            .service(routes::table::create_table)
            .service(routes::table::rename_table)
            .service(routes::table::remove_table)
            .service(routes::table::delete_table)
            .service(routes::table::duplicate_table)
            .service(routes::column::add_column)
            .service(routes::column::update_column)
            .service(routes::row::add_row)
            .service(routes::cell::update_cell)
    })
        .bind("127.0.0.1:8081")?
        .run().await
}
