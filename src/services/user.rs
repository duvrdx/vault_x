use actix_web::{
  get, post, delete, put,
  web::{Data, Json, Path},
  Responder, HttpResponse
};
use sqlx::{self};
use crate::models::user::{User, CreateUserBody, UserResponse, MasterKeyBody};
use crate::models::vault::{Vault};
use crate::models::account::{Account};
use crate::utils::cryptography::{decrypt_string};

use crate::AppState;
use bcrypt::{hash, DEFAULT_COST};

#[get("/user/{id}")]
pub async fn get_user(state: Data<AppState>, path: Path<i32>) -> impl Responder {
  let id = path.into_inner();
  
  match sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
      .bind(id)
      .fetch_one(&state.db)
      .await
  {
      Ok(user) => {
        return HttpResponse::Ok().json(user)
      },
      Err(_) => {
        println!("");
        return HttpResponse::NotFound().json("User not found")
      },
  }
}

#[get("/user")]
pub async fn get_all_users(state: Data<AppState>) -> impl Responder {
  match sqlx::query_as::<_, UserResponse>("SELECT id, username FROM users")
      .fetch_all(&state.db)
      .await
  {
      Ok(users) => {
        return HttpResponse::Ok().json(users)
      },
      Err(_) => {
        println!("");
        return HttpResponse::InternalServerError().json("Failed to get users")
      },
  }
}

#[post("/user")]
pub async fn create_user(state: Data<AppState>, body: Json<CreateUserBody>) -> impl Responder {

  let hashed_password = hash(&body.password, DEFAULT_COST).unwrap();

  match sqlx::query_as::<_, UserResponse>(
      "INSERT INTO users (username, password) VALUES ($1, $2) RETURNING id, username"
  )
      .bind(body.username.to_string())
      .bind(hashed_password)
      .fetch_one(&state.db)
      .await
  {
      Ok(users) => {
        return HttpResponse::Ok().json(users)
      },
      Err(_) => {
        println!("");
        return HttpResponse::InternalServerError().json("Failed to create users")
      },
  }
}

#[delete("/user/{id}")]
pub async fn delete_user(state: Data<AppState>, path: Path<i32>) -> impl Responder {
  let id = path.into_inner();
  
  match sqlx::query("DELETE FROM users WHERE id = $1")
      .bind(id)
      .execute(&state.db)
      .await
  {
      Ok(_) => {
        return HttpResponse::Ok().json("user deleted")
      },
      Err(_) => {
        println!("");
        return HttpResponse::InternalServerError().json("Failed to delete user")
      },
  }
}

#[put("/user/{id}")]
pub async fn update_user(state: Data<AppState>, path: Path<i32>, body: Json<CreateUserBody>) -> impl Responder {
  let id = path.into_inner();
  
  match sqlx::query("UPDATE users SET username = $1, password = $2 WHERE id = $3")
      .bind(body.username.to_string())
      .bind(body.password.to_string())
      .bind(id)
      .execute(&state.db)
      .await
  {
      Ok(_) => {
        return HttpResponse::Ok().json("user updated")
      },
      Err(_) => {
        println!("");
        return HttpResponse::InternalServerError().json("Failed to update user")
      },
  }
}

#[get("/user/{user_id}/vaults")]
pub async fn get_user_vaults(state: Data<AppState>, path: Path<i32>) -> impl Responder {
  let user_id = path.into_inner();
  
  match sqlx::query_as::<_, UserResponse>("SELECT id, title FROM vaults WHERE user_id = $1")
      .bind(user_id)
      .fetch_all(&state.db)
      .await
  {
      Ok(vaults) => {
        return HttpResponse::Ok().json(vaults)
      },
      Err(_) => {
        println!("");
        return HttpResponse::InternalServerError().json("Failed to get vaults")
      },
  }
}

#[get("/user/{user_id}/vault/{vault_id}")]
pub async fn get_user_accounts(state: Data<AppState>, path: Path<(i32, i32)>, body: Json<MasterKeyBody>) -> impl Responder {
  let (user_id, vault_id) = path.into_inner();

  let vault;

  match sqlx::query_as::<_, Vault>("SELECT * FROM vaults WHERE id = $1 AND user_id = $2")
      .bind(vault_id)
      .bind(user_id)
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

  
  match sqlx::query_as::<_, Account>("SELECT * FROM accounts WHERE vault_id = $1")
  .bind(vault_id)
  .fetch_all(&state.db)
  .await
  {
    Ok(mut accounts) => {
      if body.master_key != ""{
          if !bcrypt::verify(&body.master_key, &vault.master_key).unwrap(){
            return HttpResponse::Unauthorized().json("Invalid master key")
          }

          for account in accounts.iter_mut(){
            account.encrypted_login = decrypt_string(&account.encrypted_login.to_string(), &vault.master_key.to_string());
            account.encrypted_password = decrypt_string(&account.encrypted_password.to_string(), &vault.master_key.to_string());
          }
        }
        return HttpResponse::Ok().json(accounts)
      },
      Err(_) => {
        println!("");
        return HttpResponse::NotFound().json("Accounts not found")
      },
  }


}