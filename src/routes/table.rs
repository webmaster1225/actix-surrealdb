use actix_web::{ get, post, delete, web, HttpResponse, Responder };
use serde::{ Deserialize, Serialize };
use serde_json::{ Value, json };
use surrealdb::Error;
use surrealdb::sql::Query;
use surrealdb::sql;
use crate::DB;
use crate::model::model;
use std::sync::{ Arc, Mutex };
use crate::model::model::ContentType;

#[derive(Serialize, Deserialize)]
pub struct CreateTableRequest {
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct DuplicateRequest {
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct Table {
    name: String,
}

#[derive(Deserialize)]
struct UpdateCellRequest {
    table_name: String,
    row: usize,
    col: usize,
    content: Option<Vec<String>>,
    content_type: Option<ContentType>,
}

#[post("/create_table")]
async fn create_table(table: web::Json<CreateTableRequest>) -> impl Responder {
    // Execute the CREATE TABLE query
    let result: Result<Vec<Value>, Error> = DB.create(table.name.clone()).content(
        model::Table::default()
    ).await;
    match result {
        Ok(response) => {
            let update_result: Result<Vec<Value>, Error> = DB.update(table.name.to_string()).merge(
                json!({"name": table.name})
            ).await;
            match update_result {
                Ok(response) => HttpResponse::Ok().json(response),
                Err(e) => { HttpResponse::InternalServerError().body(format!("Error: {}", e)) }
            }
        }
        Err(e) => { HttpResponse::InternalServerError().body(format!("Error: {}", e)) }
    }
}

#[delete("/delete_table/{table_name}")]
async fn delete_table(table_name: web::Path<String>) -> impl Responder {
    let table_name = table_name.into_inner();
    let delete_result: Result<Vec<Value>, Error> = DB.delete(&table_name).await;

    match delete_result {
        Ok(response) => HttpResponse::Ok().json(format!("Table with name {} deleted", &table_name)),
        Err(e) => {
            HttpResponse::InternalServerError().body(
                format!("Table with name {} not found", table_name)
            )
        }
    }
}
