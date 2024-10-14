use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use axum::{extract::State, Json};
use serde::Deserialize;
use sqlx::query;

use crate::AppState;

#[derive(Deserialize)]
pub struct RegisterInfo {
    pub name: String,
    pub email: String,
    pub password: String,
}

pub async fn register(State(state): State<AppState>, Json(info): Json<RegisterInfo>) {
    // Generate the salt that is used.
    let salt = SaltString::generate(&mut OsRng);

    // Use default options for Argon2 algorithom
    let argon2 = Argon2::default();
    // Generate the hash
    let hash = argon2
        .hash_password(info.password.as_bytes(), &salt)
        .unwrap()
        .to_string();

    query!(
        "INSERT INTO user_info (name, email, hash) VALUES (?, ?, ?)",
        info.name,
        info.email,
        hash
    )
    .execute(&state.db)
    .await
    .unwrap();
}
