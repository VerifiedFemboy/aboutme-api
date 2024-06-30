use std::fs;

use actix_web::{App, get, HttpResponse, HttpServer};
use serde_json::Value;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config_data = fs::read_to_string("config.json").expect("Should've been able to read the config file");
    let v: Value = serde_json::from_str(&config_data).unwrap();

    if let Some(ip) = v.get("IP-HOST").and_then(Value::as_str) {
        HttpServer::new(move || App::new().service(index))
            .bind(ip)?
            .run()
            .await
    } else {
        panic!("Something went wrong while launching the API. Check the config file");
    }

}

#[get("/")]
async fn index() -> HttpResponse {
    HttpResponse::Ok().body("jaja")
}


