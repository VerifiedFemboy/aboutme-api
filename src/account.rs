use actix_web::{get, HttpResponse, post, web};
use actix_web::web::Json;
use mongodb::bson::oid::ObjectId;
use rand::Rng;
use serde::{Deserialize, Serialize};
use crate::auth::AuthKey;
use crate::database::Database;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Account {
    pub _id: u32,
    pub name: String,
    pub about: String,
    pub create_date: String,
    pub auth_key: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct AccountCreatePost {
    pub name: String
}

impl Account {
    pub async fn create(&self) -> std::io::Result<&str> {
        println!("{:?}", self);
        Ok("okie ;3")
    }

    pub fn new(_id: u32, name: String, about: String, create_date: String, auth_key: String) -> Self {
        Self { _id, name, about, create_date, auth_key }
    }
}

#[post("/account/create")]
pub async fn create_account(post_account_args: Json<AccountCreatePost>, database: web::Data<Database>) -> HttpResponse {

    let name = post_account_args.name.to_string();
    let key = AuthKey::generate_string();

    let account = Account::new(generate_id().await, name, "".to_string(), "".to_string(), key);
    match database.insert_account(account.clone()).await {
        Ok(_) => {
            HttpResponse::Ok().body(format!("Account created! This is your auth key: {}\nDo not share IT!", account.auth_key.to_string()))
        },
        Err(_) => {
            HttpResponse::InternalServerError().body("Something went wrong while creating an Account!")
        }
    }
}

#[get("/account/{account_id}")]
pub async fn get_account(account_id: web::Path<String>, database: web::Data<Database>) -> HttpResponse {
    match database.get_account_by_id(&account_id).await {
        Ok(Some(account)) => HttpResponse::Ok().json(account),
        Ok(None) => HttpResponse::NotFound().body("Account not found!"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Something went wrong! {e}"))
    }
}

async fn generate_id() -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen::<u32>()
}