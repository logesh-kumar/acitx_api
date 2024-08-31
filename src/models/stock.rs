use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Serialize, Deserialize)]
pub struct Stock {
    pub id: i32,
    pub name: String,
    pub symbol: String,
    pub similar_stocks: Option<Vec<String>>,
    pub created_at: Option<OffsetDateTime>,
    pub processed_at: Option<OffsetDateTime>,
}
