use crate::ggez_backend;
use crate::server::endpoint_handler::EndpointHandler;
use crate::server::endpoint_reciever_stack::EndpointRecieverStack;
use crate::server::endpoints;
use crate::server::json_or_form::JsonOrForm;
use crate::server::payloads::{ClearPayload, MultipleShapesPayload, ShapePayload};
use crate::server::router::ApiRouter;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

use axum::routing::post;

pub async fn run() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE) // Set the maximum log level to TRACE
        .finish();

    // Initialize the global subscriber
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let endpoint_reciever_stack = EndpointRecieverStack::new();

    let mut api_router = ApiRouter::new();

    let tx_shapes = endpoint_reciever_stack
        .shapes_reciever
        .handler
        .sender()
        .clone();

    let tx_multi_shapes = endpoint_reciever_stack
        .multi_shapes_reciever
        .handler
        .sender()
        .clone();

    let tx_clear = endpoint_reciever_stack
        .clear_reciever
        .handler
        .sender()
        .clone();

    api_router.register_endpoint::<endpoints::ShapesEndpoint, ShapePayload>(
        "/shapes".to_string(),
        post(move |payload: JsonOrForm<ShapePayload>| endpoints::handle_shapes(payload, tx_shapes)),
    );

    api_router.register_endpoint::<endpoints::ClearEndpoint, ClearPayload>(
        "/clear".to_string(),
        post(move |payload: JsonOrForm<ClearPayload>| endpoints::clear_handler(payload, tx_clear)),
    );

    api_router.register_endpoint::<endpoints::MultiShapesEndpoint, MultipleShapesPayload>(
        "/multiple_shapes".to_string(),
        post(move |payload: JsonOrForm<MultipleShapesPayload>| {
            endpoints::multiple_shapes_handler(payload, tx_multi_shapes)
        }),
    );

    let server_task = api_router.spawn();

    let run_result = ggez_backend::game::run_game(endpoint_reciever_stack);

    run_result.unwrap_err();
    server_task.await.unwrap();
}
