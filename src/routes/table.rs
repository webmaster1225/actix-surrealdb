use actix_web::{ get, post, delete, web, HttpResponse, Responder };
use serde::{ Deserialize, Serialize };
use serde_json::Value;
use surrealdb::Error;
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
async fn create_table(table: web::Json<CreateTableRequest>) -> impl Responder {
    let result: Result<Vec<Value>, Error> = DB.create(table.name.clone()).content(table).await;

    match result {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(e) => { HttpResponse::InternalServerError().body(format!("Error: {}", e)) }
    }
}

#[delete("/delete_table")]
async fn delete_table(path: web::Path<Table>) -> impl Responder {
    let table_name = path.name.clone();
    let result: Result<Vec<Value>, Error> = DB.delete(table_name).await;

    match result {
        Ok(response) => HttpResponse::Ok().json(format!("Table with name {} deleted", table_name)),
        Err(e) => {
            HttpResponse::InternalServerError().body(
                format!("Table with name {} not found", table_name)
            )
        }
    }
}
