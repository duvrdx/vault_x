use actix_web::{
  get, post, delete, put,
  web::{Data, Json, Path},
  Responder, HttpResponse
};

use sqlx::{self};
use crate::models::account::{CreateAccountBody, AccountResponse};
use crate::models::vault::{Vault};
use crate::AppState;
use crate::utils::cryptography::{encrypt_string};
use bcrypt::{verify};

#[get("/account/{id}")]
pub async fn get_account(state: Data<AppState>, path: Path<i32>) -> impl Responder {
  let id = path.into_inner();
  
  match sqlx::query_as::<_, AccountResponse>("SELECT * FROM accounts WHERE id = $1")
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
  match sqlx::query_as::<_, AccountResponse>("SELECT * FROM accounts")
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
  let vault;

  match sqlx::query_as::<_, Vault>("SELECT * FROM vaults WHERE id = $1")
      .bind(body.vault_id)
      .fetch_one(&state.db)
      .await
  {
      Ok(vault_response) => {
        vault = vault_response;
      },
      Err(_) => {
        return HttpResponse::NotFound().json("Vault not found")
      },
  }

  if bcrypt::verify(&body.master_key, &vault.master_key).unwrap() == false {
    return HttpResponse::Unauthorized().json("Invalid master key")
  }

  match sqlx::query_as::<_, AccountResponse>(
      "INSERT INTO accounts (encrypted_login, encrypted_password, vault_id, title) VALUES ($1, $2, $3, $4) RETURNING id, title, vault_id"
  )
      .bind(encrypt_string(&body.login.to_string(), &vault.master_key))
      .bind(encrypt_string(&body.password.to_string(), &vault.master_key))
      .bind(body.vault_id)
      .bind(body.title.to_string())
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

#[put("/account/{id}")]
pub async fn update_account(state: Data<AppState>, path: Path<i32>, body: Json<CreateAccountBody>) -> impl Responder {
  let id = path.into_inner();
  let vault;

  match sqlx::query_as::<_, Vault>("SELECT * FROM vaults WHERE id = $1")
      .bind(body.vault_id)
      .fetch_one(&state.db)
      .await
  {
      Ok(vault_response) => {
        vault = vault_response;
      },
      Err(_) => {
        return HttpResponse::NotFound().json("Vault not found")
      },
  }

  if verify(&body.master_key, &vault.master_key).unwrap() == false {
    return HttpResponse::Unauthorized().json("Invalid master key")
  }

  match sqlx::query_as::<_, AccountResponse>(
      "UPDATE accounts SET encrypted_login = $1, encrypted_password = $2, vault_id = $3, title = $4 WHERE id = $5 RETURNING id, title, vault_id"
  )
      .bind(encrypt_string(&body.login.to_string(), &vault.master_key))
      .bind(encrypt_string(&body.password.to_string(), &vault.master_key))
      .bind(body.vault_id)
      .bind(body.title.to_string())
      .bind(id)
      .fetch_one(&state.db)
      .await
  {
      Ok(account) => {
        println!("Account updated: {:?}", account);
        return HttpResponse::Ok().json(account)
      },
      Err(_) => {
        println!("");
        return HttpResponse::InternalServerError().json("Failed to update account")
      },
  }
}