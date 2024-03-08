use actix_web::{
  get, post, delete,
  web::{Data, Json, Path},
  Responder, HttpResponse
};
use sqlx::{self};
use crate::models::account::{Account, CreateAccountBody};
use crate::AppState;

#[get("/")]
pub async fn hello_world() -> impl Responder {
  "Hello, world!"
}

#[get("/account/{id}")]
pub async fn get_account(state: Data<AppState>, path: Path<i32>) -> impl Responder {
  let id = path.into_inner();
  
  match sqlx::query_as::<_, Account>("SELECT * FROM accounts WHERE id = $1")
      .bind(id)
      .fetch_one(&state.db)
      .await
  {
      Ok(account) => {
        println!("Account found: {:?}", account);
        return HttpResponse::Ok().json(account)
      },
      Err(_) => {
        println!("");
        return HttpResponse::NotFound().json("Account not found")
      },
  }
}

#[get("/account")]
pub async fn get_all_accounts(state: Data<AppState>) -> impl Responder {
  match sqlx::query_as::<_, Account>("SELECT * FROM accounts")
      .fetch_all(&state.db)
      .await
  {
      Ok(accounts) => {
        println!("Accounts found: {:?}", accounts);
        return HttpResponse::Ok().json(accounts)
      },
      Err(_) => {
        println!("");
        return HttpResponse::InternalServerError().json("Failed to get accounts")
      },
  }
}

#[post("/account")]
pub async fn create_account(state: Data<AppState>, body: Json<CreateAccountBody>) -> impl Responder {
  match sqlx::query_as::<_, Account>(
      "INSERT INTO accounts (username, password) VALUES ($1, $2) RETURNING id, username, password"
  )
      .bind(body.username.to_string())
      .bind(body.password.to_string())
      .fetch_one(&state.db)
      .await
  {
      Ok(account) => {
        println!("Account created: {:?}", account);
        return HttpResponse::Ok().json(account)
      },
      Err(_) => {
        println!("");
        return HttpResponse::InternalServerError().json("Failed to create account")
      },
  }
}

#[delete("/account/{id}")]
pub async fn delete_account(state: Data<AppState>, path: Path<i32>) -> impl Responder {
  let id = path.into_inner();
  
  match sqlx::query("DELETE FROM accounts WHERE id = $1")
      .bind(id)
      .execute(&state.db)
      .await
  {
      Ok(_) => {
        println!("Account deleted: {:?}", id);
        return HttpResponse::Ok().json("Account deleted")
      },
      Err(_) => {
        println!("");
        return HttpResponse::InternalServerError().json("Failed to delete account")
      },
  }
}