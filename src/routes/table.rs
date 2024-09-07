use std::default;

use actix_web::{ get, post, delete, web, HttpResponse, Responder };
use serde::{ Deserialize, Serialize };
use serde_json::Value;
use surrealdb::Error;
use crate::DB;
use crate::model::model;

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
    let result: Result<Vec<Value>, Error> = DB.create(table.name.clone()).content({
        model::Table::default()
    }).await;

    match result {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(e) => { HttpResponse::InternalServerError().body(format!("Error: {}", e)) }
    }
}

#[delete("/delete_table/{name}")]
async fn delete_table(path: web::Path<Table>) -> impl Responder {
    let table_name = path.name.clone();
    let result: Result<Vec<Value>, Error> = DB.delete(&table_name).await;

    match result {
        Ok(response) => HttpResponse::Ok().json(format!("Table with name {} deleted", &table_name)),
        Err(e) => {
            HttpResponse::InternalServerError().body(
                format!("Table with name {} not found", table_name)
            )
        }
    }
}

// pub async fn duplicate_table(
//     db: web::Data<Arc<Surreal<Mem>>>,
//     req: web::Json<DuplicateTableRequest>
// ) -> Result<impl Responder, Error> {
//     let table_name = req.table_name.clone();
//     let new_table_name = req.new_table_name.clone();

//     let table: Table = db.find(table_name).await?.into_iter().next().unwrap().into();

//     let new_table = Table {
//         name: new_table_name,
//         columns: table.columns,
//         rows: table.rows,
//     };

//     db.create(new_table_name).content(new_table).await?;

//     Ok(HttpResponse::Ok().json(json!({ "message": "Table duplicated successfully" })))
// }
