use std::env;
use tracing::info;
use tracing_subscriber::EnvFilter;
use hls_service::config::HLSConfig;
use hls_service::app_state::AppState;
use hls_service::object_database::ObjectDatabase;
use tokio::{signal};
use tokio::net::TcpListener;
use hls_service::router::init_router;

#[tokio::main(flavor = "multi_thread")]
async fn main() {

    let run_mode = env::var("HLS_MODE").unwrap_or_else(|_| "development".into());
    let config = HLSConfig::new(&run_mode).unwrap_or_else(|err| panic!("Missing needed env: {}", err));
    let filter = EnvFilter::try_new(config.log_level.clone()).unwrap_or_else(|_| EnvFilter::new("info"));
    tracing_subscriber::fmt().with_env_filter(filter).init();

    info!("Starting up ISM in {run_mode} mode.");
    //init app state and both database connections, exit application if failing
    let app_state = AppState {
        env: config.clone(),
        s3_bucket: ObjectDatabase::new(&config.object_db_config).await
    };
    let app = init_router(app_state).await;

    let url = format!("{}:{}", config.hls_url, config.hls_port);
    let listener = TcpListener::bind(url.clone()).await.unwrap();
    info!("HLS-Transcoder-Server up and is listening on: {url}");
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())//only working if there are no active connections
        .await
        .unwrap();
    info!("Stopping HLS-Server...");
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}