use std::sync::Arc;
use axum::extract::{Multipart, State};
use axum::http::{StatusCode};
use axum::Json;
use axum::response::IntoResponse;
use hlskit::models::hls_video_processing_settings::{FfmpegVideoProcessingPreset, HlsVideoAudioBitrate, HlsVideoAudioCodec, HlsVideoProcessingSettings};
use hlskit::process_video;
use log::{error, info};
use crate::app_state::AppState;
use crate::model::{MediaDto, TranscodingResponseDto};

pub async fn transcode_video(
    State(state): State<Arc<AppState>>,
    mut multipart: Multipart
) -> impl IntoResponse {
    info!("Transcoding Video started");
    let mut video_bytes: Option<Vec<u8>> = None;
    let mut metadata_dto: Option<MediaDto> = None;

    while let Some(field) = multipart.next_field().await.unwrap() {
        let field_name = match field.name() {
            Some(name) => name.to_string(),
            None => continue, // Feld ohne Namen ignorieren
        };
        match field_name.as_str() {
            "video" => video_bytes = Some(field.bytes().await.unwrap().to_vec()),
            "metadata" => {
                let json_text = field.text().await.unwrap();
                match serde_json::from_str::<MediaDto>(&json_text) {
                    Ok(dto) => {
                        metadata_dto = Some(dto);
                    }
                    Err(e) => {
                        info!("Failed to deserialize metadata: {}", e);
                        return Err((StatusCode::BAD_REQUEST, format!("Invalid JSON in 'metadata' field: {}", e)).into_response());
                    }
                }
            }
            _ => {
                info!("Ignoring unknown field: {}", field_name);
            }
        }
    }
    if let (Some(video_data), Some(dto)) = (video_bytes, metadata_dto) {
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
        match process_video(video_data, profiles).await {
            Ok(video) => {
                match state.s3_bucket.insert_object(dto.id, video).await {
                    Ok(relative_path) => {
                        info!("Uploaded Video");
                        let response = TranscodingResponseDto {video_url: relative_path};
                        Ok((StatusCode::CREATED, Json(response)).into_response())
                    },
                    Err(err) => {
                        error!("Error uploading video to S3: {:?}", err);
                        Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response())
                    }
                }
            },
            Err(err) => {
                error!("Error processing video: {:?}", err);
                Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response())
            }
        }
    } else {
        info!("Missing 'video' or 'metadata' field in the request.");
        Err((StatusCode::BAD_REQUEST, "Request must contain both 'video' and 'metadata' fields.").into_response())
    }
}
