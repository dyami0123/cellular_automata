use crate::ggez_backend::payload_to_shape::payload_to_shape;
use crate::server::endpoint_reciever_stack::EndpointRecieverStack;
use crate::server::payloads::{ClearPayload, MultipleShapesPayload, ShapePayload};
use crate::shapes;
use ggez::graphics;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use tokio::sync::mpsc;

pub struct ApiHandler {}

pub fn new_shapes_id(global_shape_counter: &Arc<Mutex<i32>>) -> i32 {
    let shape_id = {
        let mut global_shape_counter = global_shape_counter.lock().unwrap();
        *global_shape_counter += 1;
        *global_shape_counter
    };
    shape_id
}

impl ApiHandler {
    pub fn handle(
        &self,
        shapes: Arc<Mutex<HashMap<i32, Box<dyn shapes::shape::Shape>>>>,
        endpoint_reciever_stack: EndpointRecieverStack,
    ) -> (
        Arc<Mutex<HashMap<i32, Box<dyn shapes::shape::Shape>>>>,
        EndpointRecieverStack,
    ) {
        let global_shape_counter = Arc::new(Mutex::new(0));

        let (clear_rx, shapes_rx, multiple_shapes_rx) = endpoint_reciever_stack.to_recievers();

        let shapes =
            self.spawn_shapes_handler_thread(shapes, shapes_rx, global_shape_counter.clone());
        let shapes =
            self.spawn_clear_handler_thread(shapes, clear_rx, global_shape_counter.clone());

        let shapes = self.spawn_multiple_shapes_handler_thread(
            shapes,
            multiple_shapes_rx,
            global_shape_counter.clone(),
        );

        (shapes, endpoint_reciever_stack)
    }

    pub fn spawn_shapes_handler_thread(
        &self,
        shapes: Arc<Mutex<HashMap<i32, Box<dyn shapes::shape::Shape>>>>,
        shapes_rx: Arc<Mutex<mpsc::Receiver<ShapePayload>>>,
        global_shape_counter: Arc<Mutex<i32>>,
    ) -> Arc<Mutex<HashMap<i32, Box<dyn shapes::shape::Shape>>>> {
        let shapes_copy = shapes.clone();
        thread::spawn(move || loop {
            {
                let mut shapes_rx = shapes_rx.lock().unwrap();
                while let Ok(payload) = shapes_rx.try_recv() {
                    let shape_result = payload_to_shape(payload);
                    if let Ok(shape) = shape_result {
                        tracing::debug!("shape received");
                        let shape_id = new_shapes_id(&global_shape_counter);

                        if let Ok(mut shapes) = shapes.lock() {
                            shapes.insert(shape_id, shape);
                        }
                    }
                }
            }
        });
        shapes_copy
    }

    pub fn spawn_multiple_shapes_handler_thread(
        &self,
        shapes: Arc<Mutex<HashMap<i32, Box<dyn shapes::shape::Shape>>>>,
        multiple_shapes_rx: Arc<Mutex<mpsc::Receiver<MultipleShapesPayload>>>,
        global_shape_counter: Arc<Mutex<i32>>,
    ) -> Arc<Mutex<HashMap<i32, Box<dyn shapes::shape::Shape>>>> {
        let shapes_copy = shapes.clone();
        thread::spawn(move || loop {
            {
                let mut multiple_shapes_rx = multiple_shapes_rx.lock().unwrap();
                while let Ok(payloads) = multiple_shapes_rx.try_recv() {
                    tracing::debug!("multiple shape received");
                    if let Ok(mut shapes) = shapes.lock() {
                        shapes.clear();
                        tracing::debug!("shapes cleared");

                        for payload in payloads.into_iter() {
                            let shape_result = payload_to_shape(payload);
                            if let Ok(shape) = shape_result {
                                let shape_id = new_shapes_id(&global_shape_counter);

                                shapes.insert(shape_id, shape);
                            }
                        }
                    }
                }
            }
        });
        shapes_copy
    }

    pub fn spawn_clear_handler_thread(
        &self,
        shapes: Arc<Mutex<HashMap<i32, Box<dyn shapes::shape::Shape>>>>,
        clear_rx: Arc<Mutex<mpsc::Receiver<ClearPayload>>>,
        global_shape_counter: Arc<Mutex<i32>>,
    ) -> Arc<Mutex<HashMap<i32, Box<dyn shapes::shape::Shape>>>> {
        let shapes_copy = shapes.clone();
        thread::spawn(move || loop {
            {
                let mut clear_rx = clear_rx.lock().unwrap();
                while let Ok(clear_payload) = clear_rx.try_recv() {
                    if clear_payload.clear {
                        tracing::debug!("clear received");
                        if let Ok(mut shapes) = shapes.lock() {
                            shapes.clear();

                            let screen_clear_rect = shapes::rectangle_shape::RectangleShape {
                                start: (-9999.0, -9999.0),
                                end: (9999.0, 9999.0),
                                size: 0.0,
                                appearance: shapes::appearance::Appearance {
                                    color: graphics::Color::from([0.1, 0.2, 0.3, 1.0]),
                                },
                            };
                            let shape_id = new_shapes_id(&global_shape_counter);

                            shapes.insert(shape_id, Box::new(screen_clear_rect));
                        }
                    }
                }
            }
        });
        shapes_copy
    }
}
