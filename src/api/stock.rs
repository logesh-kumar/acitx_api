use crate::models::Stock;
use actix_web::{web, HttpResponse, Responder};
use reqwest::Client;
use serde::Serialize;
use sqlx::PgPool;

#[derive(Serialize)]
struct StockResponse {
    id: i32,
    symbol: String,
    name: String,
    similar_stocks: Vec<String>,
    // add other fields that you need to serialize
}

pub async fn get_stock(
    pool: web::Data<PgPool>,
    symbol: web::Path<String>,
    openai_api_key: web::Data<String>,
) -> impl Responder {
    let stock = sqlx::query_as!(
        Stock,
        "SELECT * FROM stocks WHERE symbol = $1",
        symbol.into_inner()
    )
    .fetch_one(pool.get_ref())
    .await;

    match stock {
        Ok(stock) => {
            println!("Using OpenAI API key: {}", openai_api_key.get_ref()); // Print the OpenAI API key

            let client = Client::new();
            let openai_response = client
                .post("https://api.openai.com/v1/chat/completions")
                .header(
                    "Authorization",
                    format!("Bearer {}", openai_api_key.get_ref()),
                )
                .json(&serde_json::json!({
                    "model": "gpt-3.5-turbo", // Use the appropriate model, e.g., gpt-4
                    "messages": [
                        {
                            "role": "system",
                            "content": "You are a financial assistant that provides consistent and reliable stock recommendations."
                        },
                        {
                            "role": "user",
                            "content": format!("Give me two stocks similar to {}", stock.name)
                        }
                    ],
                    "max_tokens": 100,
                    "temperature": 0, // Set temperature to 0 for deterministic results
                    "top_p": 1,       // Ensure the model considers the top 100% of choices, but with temperature 0, this has less effect
                    "n": 1            // Generate only one response
                }))
                .send()
                .await
                .unwrap()
                .json::<serde_json::Value>()
                .await
                .unwrap();

            println!("OpenAI API response: {:?}", openai_response); // Print the OpenAI API response

            let similar_stocks = openai_response["choices"][0]["message"]["content"]
                .as_str()
                .unwrap_or("")
                .split('\n')
                .map(String::from)
                .collect::<Vec<_>>();

            let updated_stock = sqlx::query!(
                "UPDATE stocks SET similar_stocks = $1 WHERE id = $2 RETURNING *",
                &similar_stocks,
                stock.id
            )
            .fetch_one(pool.get_ref())
            .await
            .unwrap();

            let response = StockResponse {
                id: updated_stock.id,
                symbol: updated_stock.symbol,
                name: updated_stock.name,
                similar_stocks: updated_stock.similar_stocks.unwrap_or_default(),
                // map other fields as needed
            };

            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            println!("Error fetching stock: {:?}", e); // Print the error to the console
            HttpResponse::NotFound().body(format!("Stock not found: {:?}", e)) // Return the error message in the response body
        }
    }
}
