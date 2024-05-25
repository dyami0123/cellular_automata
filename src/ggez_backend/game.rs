use crate::ggez_backend::api_handler::ApiHandler;
use crate::ggez_backend::main_state::MainState;
use crate::server::endpoint_reciever_stack::EndpointRecieverStack;
use ggez::event;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub fn run_game(endpoint_reciever_stack: EndpointRecieverStack) -> Result<(), ggez::GameError> {
    let cb = ggez::ContextBuilder::new("super_simple", "ggez");
    let (ctx, event_loop) = cb.build()?;

    let shapes = Arc::new(Mutex::new(HashMap::new()));

    let api_handler = ApiHandler {};
    let (shapes, endpoint_reciever_stack) = api_handler.handle(shapes, endpoint_reciever_stack);

    let state = MainState::new(shapes, endpoint_reciever_stack)?;
    event::run(ctx, event_loop, state)
}
