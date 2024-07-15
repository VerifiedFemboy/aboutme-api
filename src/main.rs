use std::fs;

use actix_web::{App, get, HttpResponse, HttpServer};
use actix_web::web::Data;
use serde_json::Value;
use crate::account::{create_account, get_account};
use crate::database::Database;

mod account;
mod database;
mod auth;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config_data = fs::read_to_string("config.json").expect("Should've been able to read the config file");
    let v: Value = serde_json::from_str(&config_data).unwrap();

    if let Some(mongo) = v.get("MONGO-URI").and_then(Value::as_str) {
        let db = Database::new(mongo, "server-db").await.expect("Failed with MongoDB connection...");
        if let Some(ip) = v.get("IP-HOST").and_then(Value::as_str) {
            HttpServer::new(move || App::new()
                .app_data(Data::new(db.clone()))
                .service(index)
                .service(get_account)
                .service(create_account))
                .bind(ip)?
                .run()
                .await
        } else {
            panic!("Something went wrong while launching the API. Check the config file");
        }

    } else {
        panic!("Something went wrong with mongo uri. Check the config file")
    }
}

#[get("/")]
async fn index() -> HttpResponse {
    HttpResponse::Ok().body("jaja")
}


