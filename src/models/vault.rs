use sqlx::{self, FromRow};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Vault{
    pub id: i32,
    pub title: String,
    pub master_key: String,
    pub user_id: i32
}

#[derive(Deserialize)]
pub struct CreateVaultBody{
    pub title: String,
    pub master_key: String,
    pub user_id: i32
}

#[derive(Serialize, FromRow)]
pub struct VaultResponse{
    pub id: i32,
    pub title: String,
    pub user_id: i32
}