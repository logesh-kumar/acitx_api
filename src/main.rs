mod api;
mod middlewares;
mod models;
mod services;
mod utils;

use actix_web::{middleware::Logger, web, App, HttpServer};

use dotenv::dotenv;
use env_logger::Env;
use middlewares::JwtMiddleware;
use sqlx::postgres::PgPoolOptions;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize the logger
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    // Load environment variables from .env file
    dotenv().ok();

    // Retrieve the database URL from environment variables
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Create a connection pool using the DATABASE_URL
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool");

    // Retrieve the OpenAI API key from environment variables
    let openai_api_key = env::var("OPAPI_KEY").expect("OPAPI_KEY must be set");

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(openai_api_key.clone())) // Pass API key as application data
            // Public routes
            .route("/api/register", web::post().to(api::register))
            .route("/api/login", web::post().to(api::login))
            // Protected routes with middleware
            .service(
                web::scope("/api")
                    .wrap(JwtMiddleware)
                    .route("/greet/{name}/{age}", web::get().to(api::greet))
                    .route("/stocks/{symbol}", web::get().to(api::get_stock))
                    .route("/upload", web::post().to(api::upload_video))
                    .route("/protected", web::get().to(api::protected)),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
