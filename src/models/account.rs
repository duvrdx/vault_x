use sqlx::{self, FromRow};
use serde::{Deserialize, Serialize};

#[derive(Serialize, FromRow, Debug)]
pub struct Account {
    id: i32,
    username: String,
    password: String,
}

#[derive(Deserialize)]
pub struct CreateAccountBody{
  pub username: String,
  pub password: String,
}