
use actix_web::{web::Data, App, HttpServer};
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use bcrypt::{verify, DEFAULT_COST};

mod services;
mod models; 
mod utils;

use services::{vault, user};
use utils::cryptography::{encrypt_string, decrypt_string};

struct AppState{
    db: Pool<Postgres>
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    dotenv().ok();

    // Configurações do .env   
    let database_url: String = std::env::var("DATABASE_URL").expect("DATABASE_URL is required");
    let server_url: String = std::env::var("SERVER_URL").expect("SERVER_URL is required");  
    
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to Postgres");


    println!("\n\nPassword Vault is Running at http://{}:7777", server_url);
    
    return HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState { db: pool.clone() }))
            .service(user::get_user)
            .service(user::get_all_users)
            .service(user::create_user)
            .service(user::delete_user)
            .service(user::update_user)
            .service(vault::get_vault)
            .service(vault::get_all_vaults)
            .service(vault::create_vault)
            .service(vault::delete_vault)
            .service(vault::update_vault)
    })
    .bind((server_url, 7777))?
    .run()
    .await;

}