use actix_web::{ post, web, HttpResponse, Responder };
use serde::Deserialize;
use serde_json::Value;
use surrealdb::Error;
use crate::{ model::model::Cell, DB };
use crate::model::model::{ ContentType, Table };

#[derive(Deserialize)]
struct AddColumnRequest {
    table_name: String,
    col: usize,
    content_type: ContentType,
}

#[derive(Deserialize)]
struct UpdateColumnRequest {
    table_name: String,
    col: usize,
    content_type: ContentType,
}

#[post("/add_column")]
async fn add_column(req: web::Json<AddColumnRequest>) -> impl Responder {
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

            if req.col < table.data[0].len() {
                // Insert the column
                for i in 0..table.data.len() {
                    table.data[i].insert(req.col, Cell {
                        content: vec![],
                        content_type: req.content_type.clone(),
                    });
                }
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
                HttpResponse::BadRequest().body("Invalid column index")
            }
        }
        None => HttpResponse::NotFound().body("Table not found"),
    }
}

#[post("/update_column")]
async fn update_column(req: web::Json<UpdateColumnRequest>) -> impl Responder {
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
            if req.col < table.data[0].len() {
                // Update the specified column
                for i in 0..table.data.len() {
                    table.data[i][req.col] = Cell {
                        content: vec![],
                        content_type: req.content_type.clone(),
                    };
                }
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
                HttpResponse::BadRequest().body("Invalid column index")
            }
        }
        None => HttpResponse::NotFound().body("Table not found"),
    }
}
