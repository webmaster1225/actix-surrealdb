use actix_web::{ get, post, web, App, HttpResponse, HttpServer, Responder };
use once_cell::sync::Lazy;
use serde::{ Deserialize, Serialize };
use serde_json::Value;
use surrealdb::engine::remote::ws::{ Client, Ws };
use surrealdb::{ Surreal, Error };
use surrealdb::opt::auth::Root;
use crate::DB;

#[derive(Serialize, Deserialize)]
pub struct CreateTableRequest {
    pub name: String,
}

#[derive(Serialize, Deserialize)]
struct Table {
    name: String,
}

#[post("/create_table")]
pub async fn create_table(table: web::Json<CreateTableRequest>) -> impl Responder {
    let result: Result<Vec<Value>, Error> = DB.create("table").content(table).await;

    match result {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(e) => { HttpResponse::InternalServerError().body(format!("Error: {}", e)) }
    }
}

#[delete("/remove_table")]
pub async fn remove_table(table: web::Json<CreateTableRequest>) -> impl Responder {
    let result: Result<Vec<Value>, Error> = DB.create("table").content(table).await;

    match result {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(e) => { HttpResponse::InternalServerError().body(format!("Error: {}", e)) }
    }
}
