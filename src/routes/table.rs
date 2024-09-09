use actix_web::{ post, delete, web, HttpResponse, Responder };
use serde::{ Deserialize, Serialize };
use serde_json::{ Value, json };
use surrealdb::Error;
use crate::DB;
use crate::model::model;
use crate::model::model::Table;

#[derive(Serialize, Deserialize)]
pub struct CreateTableRequest {
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct RenameTableRequest {
    pub new_table_name: String,
    pub old_table_name: String,
}
#[derive(Deserialize)]
struct DuplicateRequest {
    table_name: String,
}

#[post("/create_table")]
async fn create_table(table: web::Json<CreateTableRequest>) -> impl Responder {
    // Execute the CREATE TABLE query
    let result: Result<Vec<Value>, Error> = DB.create(table.name.clone()).content(
        model::Table::default()
    ).await;
    match result {
        Ok(_) => {
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

#[post("/rename_table")]
async fn rename_table(req: web::Json<RenameTableRequest>) -> impl Responder {
    let old_table_name = &req.old_table_name;
    let new_table_name = &req.new_table_name;

    // Step 1: Copy data from old table to new table
    let copy_data = format!("INSERT INTO {} SELECT * FROM {};", new_table_name, old_table_name);
    if let Err(e) = DB.query(copy_data).await {
        return HttpResponse::InternalServerError().body(format!("Failed to copy data: {}", e));
    }

    // Step 2: Drop old table
    let drop_old_table = format!("REMOVE TABLE {};", old_table_name);
    match DB.query(drop_old_table).await {
        Ok(_) =>
            HttpResponse::Ok().body(
                format!("Table '{}' renamed to '{}'", old_table_name, new_table_name)
            ),
        Err(e) =>
            HttpResponse::InternalServerError().body(format!("Failed to drop old table: {}", e)),
    }
}

#[delete("/remove_table/{table_name}")]
async fn remove_table(table_name: web::Path<String>) -> impl Responder {
    let table_name = table_name.into_inner();
    let delete_result: Result<Vec<Value>, Error> = DB.delete(&table_name).await;

    match delete_result {
        Ok(_) => HttpResponse::Ok().json(format!("Table with name {} removed", &table_name)),
        Err(_e) => {
            HttpResponse::InternalServerError().body(
                format!("Table with name {} not found", table_name)
            )
        }
    }
}

#[delete("/delete_table/{table_name}")]
async fn delete_table(table_name: web::Path<String>) -> impl Responder {
    let table_name = table_name.into_inner();

    // Construct the SQL query
    let query = format!("REMOVE TABLE `{}`", table_name);

    // Execute the query
    let delete_result = DB.query(query).await;

    match delete_result {
        Ok(_) =>
            HttpResponse::Ok().json(
                format!("Table with name '{}' deleted successfully", table_name)
            ),
        Err(e) =>
            HttpResponse::InternalServerError().body(
                format!("Failed to delete table '{}': {}", table_name, e)
            ),
    }
}

#[post("/duplicate_table")]
async fn duplicate_table(req: web::Json<DuplicateRequest>) -> impl Responder {
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
        Some(table) => {
            let duplicate_table = table.clone();
            let new_table_name = format!("{}_copy", table.name);

            let create_result: Result<Vec<Value>, Error> = DB.create(new_table_name).content(
                duplicate_table
            ).await;

            match create_result {
                Ok(_) => HttpResponse::Ok().json(json!({"status": "success"})),
                Err(e) =>
                    HttpResponse::InternalServerError().body(
                        format!("Error duplicating table: {}", e)
                    ),
            }
        }
        None => HttpResponse::NotFound().body("Table not found"),
    }
}
