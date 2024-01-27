mod api;
mod file;
mod handler;
use std::time::Duration;

use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};
use tokio::signal;
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "file_comments=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer().without_time())
        .init();

    let app = Router::new()
        .route("/ping", get(ping))
        .route("/comments", get(handler::get_comments))
        .layer((
            TraceLayer::new_for_http(),
            TimeoutLayer::new(Duration::from_secs(10)),
        ))
        .fallback(handler_404);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

/// response to health check
async fn ping() -> &'static str {
    "pong"
}

/// not found handler
async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "FBI is watching yuu!!")
}

// gracefully shutdown handler
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

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
