use crate::models::User;
use crate::utils::create_jwt;
use actix_web::{web, Error, HttpResponse};
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Deserialize)]
pub struct AuthData {
    username: String,
    password: String,
}

pub async fn register(
    pool: web::Data<PgPool>,
    auth_data: web::Json<AuthData>,
) -> Result<HttpResponse, Error> {
    let user = User::create(&pool, &auth_data.username, &auth_data.password)
        .await
        .map_err(|_| HttpResponse::InternalServerError().finish())
        .unwrap();

    Ok(HttpResponse::Ok().json(user))
}

pub async fn login(
    pool: web::Data<PgPool>,
    auth_data: web::Json<AuthData>,
) -> Result<HttpResponse, Error> {
    if let Some(user) = User::find_by_username(&pool, &auth_data.username)
        .await
        .unwrap()
    {
        if user.verify_password(&auth_data.password) {
            let token = create_jwt(&user.username);
            return Ok(HttpResponse::Ok().json(token));
        }
    }

    Ok(HttpResponse::Unauthorized().body("Invalid credentials"))
}

pub async fn protected() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().body("Access granted"))
}
