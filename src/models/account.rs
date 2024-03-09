use sqlx::{self, FromRow};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Account{
  pub id: i32,
  pub title: String,
  pub encrypted_login: String,
  pub encrypted_password: String,
  pub vault_id: i32,
}

#[derive(Deserialize)]
pub struct CreateAccountBody{
  pub title: String,
  pub login: String,
  pub password: String,
  pub vault_id: i32,
  pub master_key: String
}

#[derive(Debug, Serialize, FromRow)]
pub struct AccountResponse{
  pub id: i32,
  pub title: String,
  pub vault_id: i32,
}