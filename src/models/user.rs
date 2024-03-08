use sqlx::{self, FromRow};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct User{
    pub id: i32,
    pub username: String,
    pub password: String
}

#[derive(Deserialize)]
pub struct CreateUserBody{
    pub username: String,
    pub password: String
}

#[derive(Serialize, FromRow)]
pub struct UserResponse{
    pub id: i32,
    pub username: String
}