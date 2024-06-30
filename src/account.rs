use actix_web::{HttpResponse, post, web};
use actix_web::web::Json;
use serde::{Deserialize, Serialize};

use crate::database::Database;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Account {
    pub name: String,
    pub about: String,
    pub create_date: String,
    pub auth_key: String
}

impl Account {
    pub async fn create(&self) -> std::io::Result<&str> {
        println!("{:?}", self);
        Ok("okie ;3")
    }

    pub async fn new(name: String, about: String, create_date: String, auth_key: String) -> Self {
        Self { name, about, create_date, auth_key }
    }

}

#[post("/account/create")]
pub async fn create_account(post_account_args: Json<Account>, database: web::Data<Database>) -> HttpResponse {

    let name = post_account_args.name.to_string();
    let key = "generated".to_string();

    let account = Account::new(name, "".to_string(), "".to_string(), key).await;
    match database.insert_account(account.clone()).await {
        Ok(_) => {
            HttpResponse::Ok().body(format!("Account created! This is your auth key: {}", account.auth_key.to_string()))
        },
        Err(_) => {
            HttpResponse::InternalServerError().body("Something went wrong while creating an Account!")
        }
    }
}