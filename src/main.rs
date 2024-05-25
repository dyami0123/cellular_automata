pub mod ggez_backend;
pub mod rendering;
pub mod server;
pub mod shapes;

use crate::server::payloads::{ClearPayload, ShapePayload};
use axum::{
    async_trait,
    extract::{FromRequest, Request},
    http::{header::CONTENT_TYPE, StatusCode},
    response::{IntoResponse, Response},
    routing::post,
    Form, Json, RequestExt, Router,
};
use tokio::sync::mpsc;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    server::server::run().await;
}
