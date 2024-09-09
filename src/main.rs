mod model;
mod routes;
mod db;
use routes::table;
use actix_web::{ App, HttpServer };
use surrealdb::opt::auth::Root;
use surrealdb::{ engine::remote::ws::Ws, Surreal };
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
        App::new().service(table::create_table).service(table::delete_table)
        // .service(table::duplicate_table)
        // .service(table::update_cell)
    })
        .bind("127.0.0.1:8081")?
        .run().await
}
