use crate::models::ProcessedStock;
use crate::services::video_processing::{add_audio_to_video, post_to_instagram};
use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;

#[derive(serde::Deserialize)]
pub struct UploadRequest {
    pub stock_id: i32,
    pub mp4_url: String,
    pub thumbnail_url: String,
}

pub async fn upload_video(
    pool: web::Data<PgPool>,
    req: web::Json<UploadRequest>,
) -> impl Responder {
    let UploadRequest {
        stock_id,
        mp4_url,
        thumbnail_url,
    } = req.into_inner();

    // Process the video by adding audio
    let processed_video_url =
        add_audio_to_video(&mp4_url, "background_music.mp3", "output_with_audio.mp4");

    let processed_stock = sqlx::query_as!(
        ProcessedStock,
        "INSERT INTO processed_stocks (stock_id, mp4_url, thumbnail_url) VALUES ($1, $2, $3) RETURNING *",
        stock_id,
        processed_video_url,
        thumbnail_url
    )
    .fetch_one(pool.get_ref())
    .await
    .unwrap();

    post_to_instagram(
        &processed_stock.mp4_url,
        &processed_stock.thumbnail_url,
        &format!("Stock comparison for ID: {}", stock_id),
    )
    .await;

    HttpResponse::Ok().json(processed_stock)
}
