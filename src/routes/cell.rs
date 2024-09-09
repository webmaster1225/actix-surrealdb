use actix_web::{ put, web, HttpResponse, Responder };
use serde::Deserialize;
use serde_json::json;
use surrealdb::Error;
use crate::DB;
use crate::model::model::{ ContentType, Table };

#[derive(Deserialize)]
struct UpdateCellRequest {
    table_name: String,
    row: usize,
    col: usize,
    new_content: Vec<String>,
    new_content_type: ContentType,
}

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
