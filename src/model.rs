use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TranscodingResponseDto {
    pub video_url: String
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct MediaDto {
    pub id: Uuid,
    #[serde(rename = "type")]
    pub media_type: String,
    pub created_at: DateTime<Utc>,
    pub url: String
}
