use actix_web::{ post, web, HttpResponse, Responder };
use serde::Deserialize;
use serde_json::json;
use surrealdb::Error;
use crate::DB;
use crate::model::model::Table;

#[derive(Deserialize)]
struct AddRowRequest {
    table_name: String,
    row: usize,
}

#[derive(Deserialize)]
struct DeleteRowRequest {
    table_name: String,
    row: usize,
}

#[post("/add_row")]
async fn add_row(req: web::Json<AddRowRequest>) -> impl Responder {
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
            if req.row < table.data[0].len() {
                // Insert the row
                let mut new_row = table.data[0].clone();
                for i in 0..table.data.len() {
                    new_row[i].content = vec![];
                }
                table.data.insert(req.row, new_row);
                // Save the updated table back to the database
                let update_result: Result<Vec<Table>, Error> = DB.update(
                    (&req.table_name).clone()
                ).content(table).await;

                match update_result {
                    Ok(_) => HttpResponse::Ok().json(json!({"status": "success"})),
                    Err(e) =>
                        HttpResponse::InternalServerError().body(
                            format!("Error addubg row: {}", e)
                        ),
                }
            } else {
                HttpResponse::BadRequest().body("Invalid row index")
            }
        }
        None => HttpResponse::NotFound().body("Table not found"),
    }
}

#[post("/delete_row")]
async fn delete_row(req: web::Json<DeleteRowRequest>) -> impl Responder {
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
            if req.row < table.data.len() {
                table.data.remove(req.row);
                // Save the updated table back to the database
                let update_result: Result<Vec<Table>, Error> = DB.update(
                    (&req.table_name).clone()
                ).content(table).await;

                match update_result {
                    Ok(_) => HttpResponse::Ok().json(json!({"status": "success"})),
                    Err(e) =>
                        HttpResponse::InternalServerError().body(
                            format!("Error deleting row: {}", e)
                        ),
                }
            } else {
                HttpResponse::BadRequest().body("Invalid row index")
            }
        }
        None => HttpResponse::NotFound().body("Table not found"),
    }
}
