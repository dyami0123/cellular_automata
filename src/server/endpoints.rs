use crate::server::endpoint_handler::EndpointHandler;
use crate::server::json_or_form::JsonOrForm;
use crate::server::payloads::{ClearPayload, MultipleShapesPayload, ShapePayload};

use tokio::sync::mpsc;

pub async fn handle_shapes(
    JsonOrForm(payload): JsonOrForm<ShapePayload>,
    tx: mpsc::Sender<ShapePayload>,
) {
    if let Err(e) = tx.send(payload).await {
        tracing::error!("failed to send shapes payload: {}", e);
    }
}

pub async fn multiple_shapes_handler(
    JsonOrForm(payload): JsonOrForm<MultipleShapesPayload>,
    tx: mpsc::Sender<MultipleShapesPayload>,
) {
    if let Err(e) = tx.send(payload).await {
        tracing::error!("failed to send shapes payload: {}", e);
    }
}

pub async fn clear_handler(
    JsonOrForm(payload): JsonOrForm<ClearPayload>,
    tx: mpsc::Sender<ClearPayload>,
) {
    if let Err(e) = tx.send(payload).await {
        tracing::error!("failed to send clear payload: {}", e);
    }
}

pub struct ShapesEndpoint {
    sender: mpsc::Sender<ShapePayload>,
}

impl EndpointHandler<ShapePayload> for ShapesEndpoint {
    fn sender(&self) -> &mpsc::Sender<ShapePayload> {
        &self.sender
    }
    fn new(sender: mpsc::Sender<ShapePayload>) -> Self {
        Self { sender }
    }
}

pub struct ClearEndpoint {
    sender: mpsc::Sender<ClearPayload>,
}

impl EndpointHandler<ClearPayload> for ClearEndpoint {
    fn sender(&self) -> &mpsc::Sender<ClearPayload> {
        &self.sender
    }
    fn new(sender: mpsc::Sender<ClearPayload>) -> Self {
        Self { sender }
    }
}

pub struct MultiShapesEndpoint {
    sender: mpsc::Sender<MultipleShapesPayload>,
}

impl EndpointHandler<MultipleShapesPayload> for MultiShapesEndpoint {
    fn sender(&self) -> &mpsc::Sender<MultipleShapesPayload> {
        &self.sender
    }
    fn new(sender: mpsc::Sender<MultipleShapesPayload>) -> Self {
        Self { sender }
    }
}
