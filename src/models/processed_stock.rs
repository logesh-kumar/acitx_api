use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Serialize, Deserialize)]
pub struct ProcessedStock {
    pub id: i32,
    pub stock_id: Option<i32>,
    pub mp4_url: String,
    pub thumbnail_url: String,
    pub created_at: Option<OffsetDateTime>,
}
