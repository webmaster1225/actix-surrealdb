use actix_web::{ get, post, put, delete, web, HttpResponse, Responder };
use serde::{ Deserialize, Serialize };
use serde_json::{ Value, json };
use surrealdb::Error;
use surrealdb::sql::Query;
use surrealdb::sql;
use crate::DB;
use crate::model::model;
use std::sync::{ Arc, Mutex };
use crate::model::model::{ ContentType, Table };

#[derive(Serialize, Deserialize)]
pub struct CreateTableRequest {
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct DuplicateRequest {
    pub name: String,
}
#[derive(Deserialize)]
struct UpdateCellRequest {
    table_name: String,
    row: usize,
    col: usize,
    new_content: Vec<String>,
    new_content_type: ContentType,
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
// #[put("/update_cell")]
// async fn update_cell(req: web::Json<UpdateCellRequest>) -> impl Responder {
//     // Retrieve the table
//     let table_result: Result<Vec<Table>, Error> = DB.select((&req.table_name).clone()).await;
//     let table_option: Option<Table> = match table_result {
//         mut vec if vec.len() == 1 => Some(vec.remove(0)),
//         _ => None,
//     };

// }
#[put("/update_cell")]
async fn update_cell(req: web::Json<UpdateCellRequest>) -> impl Responder {
    // Retrieve the table
    let table_result: Result<Vec<Table>, Error> = DB.select((&req.table_name).clone()).await;

    let table_option: Option<Table> = match table_result {
        Ok(mut vec) if vec.len() == 1 => Some(vec.remove(0)),
        Ok(_) => None,
        Err(e) => {
            return HttpResponse::InternalServerError().body(format!("Error fetching table: {}", e));
        }
    };

    match table_option {
        Some(mut table) => {
            // Ensure the provided row and column indices are within bounds
            if req.row < table.data.len() && req.col < table.data[req.row].len() {
                // Update the specified cell
                table.data[req.row][req.col].content = req.new_content.clone();
                table.data[req.row][req.col].content_type = req.new_content_type.clone();

                // Save the updated table back to the database
                let update_result: Result<Vec<Table>, Error> = DB.update(
                    (&req.table_name).clone()
                ).content(table).await;

                match update_result {
                    Ok(_) => HttpResponse::Ok().json(json!({"status": "success"})),
                    Err(e) =>
                        HttpResponse::InternalServerError().body(
                            format!("Error updating table: {}", e)
                        ),
                }
            } else {
                HttpResponse::BadRequest().body("Invalid row or column index")
            }
        }
        None => HttpResponse::NotFound().body("Table not found"),
    }
}
