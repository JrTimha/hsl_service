use std::sync::Arc;
use axum::extract::{Multipart, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use hlskit::models::hls_video_processing_settings::{FfmpegVideoProcessingPreset, HlsVideoAudioBitrate, HlsVideoAudioCodec, HlsVideoProcessingSettings};
use hlskit::process_video;
use log::{error, info};
use uuid::Uuid;
use crate::app_state::AppState;

pub async fn transcode_video(
    State(state): State<Arc<AppState>>,
    mut multipart: Multipart
) -> impl IntoResponse {
    let mut video_bytes = Vec::new();
    while let Some(field) = multipart.next_field().await.unwrap() {
        if field.name() == Some("video") {
            video_bytes = field.bytes().await.unwrap().to_vec();
            break;
        }
    }
    let profiles = vec![
        HlsVideoProcessingSettings {
            resolution: (1920, 1080),
            constant_rate_factor: 28,
            audio_codec: HlsVideoAudioCodec::Aac,
            audio_bitrate: HlsVideoAudioBitrate::Medium,
            preset: FfmpegVideoProcessingPreset::Fast,
        },
        HlsVideoProcessingSettings {
            resolution: (1280, 720),
            constant_rate_factor: 28,
            audio_codec: HlsVideoAudioCodec::Aac,
            audio_bitrate: HlsVideoAudioBitrate::Medium,
            preset: FfmpegVideoProcessingPreset::Fast,
        },
    ];
    match process_video(video_bytes, profiles).await {
        Ok(video) => {
            match state.s3_bucket.insert_object(Uuid::new_v4(), video).await {
                Ok(relative_path) => {
                    info!("Uploaded Video");
                    (StatusCode::CREATED, relative_path).into_response()
                },
                Err(err) => {
                    error!("Error uploading video to S3: {:?}", err);
                    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response()
                }
            }
        },
        Err(err) => {
            error!("Error processing video: {:?}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response()
        }
    };
}
