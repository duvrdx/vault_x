use actix_web::{
  get, post, delete, put,
  web::{Data, Json, Path},
  Responder, HttpResponse
};
use sqlx::{self};
use crate::models::vault::{CreateVaultBody, VaultResponse};
use crate::AppState;
use bcrypt::{hash, DEFAULT_COST};


#[get("/vault/{id}/accounts")]
pub async fn get_accounts_by_vault_id(state: Data<AppState>, path: Path<i32>) -> impl Responder {
  let id = path.into_inner();
  
  match sqlx::query_as::<_, VaultResponse>("SELECT * FROM accounts WHERE vault_id = $1")
      .bind(id)
      .fetch_one(&state.db)
      .await
  {
      Ok(accounts) => {
        return HttpResponse::Ok().json(accounts)
      },
      Err(_) => {
        println!("");
        return HttpResponse::NotFound().json("Vault not found")
      },
  }
}

#[get("/vault/{id}")]
pub async fn get_vault(state: Data<AppState>, path: Path<i32>) -> impl Responder {
  let id = path.into_inner();
  
  match sqlx::query_as::<_, VaultResponse>("SELECT * FROM vaults WHERE id = $1")
      .bind(id)
      .fetch_one(&state.db)
      .await
  {
      Ok(vault) => {
        return HttpResponse::Ok().json(vault)
      },
      Err(_) => {
        println!("");
        return HttpResponse::NotFound().json("Vault not found")
      },
  }
}

#[get("/vault")]
pub async fn get_all_vaults(state: Data<AppState>) -> impl Responder {
  match sqlx::query_as::<_, VaultResponse>("SELECT id, title, user_id FROM vaults")
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

#[post("/vault")]
pub async fn create_vault(state: Data<AppState>, body: Json<CreateVaultBody>) -> impl Responder {

  let hashed_master_key = hash(&body.master_key, DEFAULT_COST).unwrap();

  println!("Master Key: {}", hashed_master_key);

  match sqlx::query_as::<_, VaultResponse>(
      "INSERT INTO vaults (title, master_key, user_id) VALUES ($1, $2, $3) RETURNING id, title, user_id"
  )
      .bind(body.title.to_string())
      .bind(hashed_master_key)
      .bind(body.user_id)
      .fetch_one(&state.db)
      .await
  {
      Ok(vaults) => {
        return HttpResponse::Ok().json(vaults)
      },
      Err(_) => {
        println!("");
        return HttpResponse::InternalServerError().json("Failed to create users")
      },
  }
}

#[delete("/vault/{id}")]
pub async fn delete_vault(state: Data<AppState>, path: Path<i32>) -> impl Responder {
  let id = path.into_inner();
  
  match sqlx::query("DELETE FROM vaults WHERE id = $1")
      .bind(id)
      .execute(&state.db)
      .await
  {
      Ok(_) => {
        return HttpResponse::Ok().json("vault deleted")
      },
      Err(_) => {
        println!("");
        return HttpResponse::InternalServerError().json("Failed to delete vault")
      },
  }
}

#[put("/vault/{id}")]
pub async fn update_vault(state: Data<AppState>, path: Path<i32>, body: Json<CreateVaultBody>) -> impl Responder {
  let id = path.into_inner();
  
  match sqlx::query("UPDATE vaults SET title = $1, master_key = $2, user_id = $3 WHERE id = $4")
      .bind(body.title.to_string())
      .bind(body.master_key.to_string())
      .bind(body.user_id)
      .bind(id)
      .execute(&state.db)
      .await
  {
      Ok(_) => {
        return HttpResponse::Ok().json("vault updated")
      },
      Err(_) => {
        println!("");
        return HttpResponse::InternalServerError().json("Failed to update vault")
      },
  }
}