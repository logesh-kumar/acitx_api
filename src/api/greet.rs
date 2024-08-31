use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use std::str::FromStr;

#[derive(serde::Serialize)]
struct GreetResponse {
    message: String,
}

#[derive(Deserialize)]
pub struct GreetParams {
    name: String,
    age: String,
}

pub async fn greet(path: web::Path<GreetParams>) -> impl Responder {
    let age = match u32::from_str(&path.age) {
        Ok(age) => age,
        Err(_) => {
            return HttpResponse::BadRequest().body("Invalid age");
        }
    };

    let json_response = GreetResponse {
        message: format!("Hello, {}! You are {} years old.", path.name, age),
    };

    HttpResponse::Ok().json(json_response)
}
