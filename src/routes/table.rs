use std::default;

use actix_web::{ get, post, delete, web, HttpResponse, Responder };
use serde::{ Deserialize, Serialize };
use serde_json::{ Value, from_value, json };
use surrealdb::sql::Object;
use surrealdb::{ Error, Response };
use crate::DB;
use crate::model::model;
use std::fmt::Display;

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

#[derive(Serialize, Deserialize)]
pub struct UpdateRequest {
    pub name: String,
    pub col: i32,
    pub row: i32,
    pub data: String,
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

// #[post("/duplicate_table")]
// async fn duplicate_table(table: web::Json<DuplicateRequest>) -> impl Responder {
//     let old_table = DB.select(table.name);
//     // let new_table = model::Table {
//     //     name: old_table.name.to_string() + "_copy",
//     //     data: old_table.data,
//     // };
//     // let duplicate_result: Result<Vec<Value>, Error> = DB.create(table.name.clone()).content(
//     //     model::Table::default()
//     // ).await;

//     // let result: Result<Vec<Value>, Error> = DB.clone_from().await;
//     // match duplicate_result {
//     //     Ok(response) => HttpResponse::Ok().json(response),
//     //     Err(e) => { HttpResponse::InternalServerError().body(format!("Error: {}", e)) }
//     // }
//     let table_name = table.name.into_inner();
//     let delete_result: Result<Vec<Value>, Error> = DB.delete(&table_name).await;

//     match delete_result {
//         Ok(response) => HttpResponse::Ok().json(format!("Table with name {} deleted", &table_name)),
//         Err(e) => {
//             HttpResponse::InternalServerError().body(
//                 format!("Table with name {} not found", table_name)
//             )
//         }
//     }
// }

#[post("/update_table")]
async fn update_table(req: web::Json<UpdateRequest>) -> impl Responder {
    let record_id = "Table2:7hqwbkvulhk32xr7npbq";
    let outer_index = "1";
    let inner_index = "1";
    let new_value = "success================>";
    let sql = format!(
        "UPDATE {} SET data[{}][{}] = '{}' WHERE id = '{}'",
        record_id,
        outer_index,
        inner_index,
        new_value,
        record_id
    );
    let update_result = DB.execute(Sql::from(sql)).await;
    match update_result {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(e) => { HttpResponse::InternalServerError().body(format!("Error: {}", e)) }
    }
}
