use std::sync::Arc;
use axum::extract::DefaultBodyLimit;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Router;
use axum::routing::{get, post};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use crate::app_state::AppState;
use crate::transcoding::transcode_video;

/**
 * Initializing the api routes.
 */
pub async fn init_router(app_state: AppState) -> Router {

    let public_routing = Router::new()
        .route("/", get(|| async { "Hello, world! I'm your new HSL-Service. 🤗" }))
        .route("/health", get(|| async { (StatusCode::OK, "Healthy").into_response() }));

    let protected_routing = Router::new() //add new routes here
        .route("/api/transcode", post(transcode_video))

        //layering bottom to top middleware
        .layer(
            ServiceBuilder::new() //layering top to bottom middleware
                .layer(TraceLayer::new_for_http()) //1
                .layer(DefaultBodyLimit::max(50 * 1024 * 1024)) //max 50mb files
        )
        .with_state(Arc::new(app_state));
    public_routing.merge(protected_routing)
}