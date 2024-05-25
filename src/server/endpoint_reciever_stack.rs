use crate::server::endpoint_handler::EndpointHandler;
use crate::server::endpoints::{ClearEndpoint, MultiShapesEndpoint, ShapesEndpoint};
use crate::server::payloads::{ClearPayload, MultipleShapesPayload, Payload, ShapePayload};
use std::clone::Clone;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use tokio::sync::mpsc::Receiver;

pub struct EndpointReciever<T, P>
where
    T: EndpointHandler<P>,
    P: Payload,
{
    pub reciever: Arc<Mutex<mpsc::Receiver<P>>>,
    pub handler: T,
}

pub struct EndpointRecieverStack {
    pub clear_reciever: EndpointReciever<ClearEndpoint, ClearPayload>,
    pub shapes_reciever: EndpointReciever<ShapesEndpoint, ShapePayload>,
    pub multi_shapes_reciever: EndpointReciever<MultiShapesEndpoint, MultipleShapesPayload>,
}

impl EndpointRecieverStack {
    pub fn new() -> EndpointRecieverStack {
        let (clear_tx, clear_rx) = mpsc::channel(32);
        let (shapes_tx, shapes_rx) = mpsc::channel(32);
        let (multi_shapes_tx, multi_shapes_rx) = mpsc::channel(32);

        let clear_reciever = EndpointReciever {
            reciever: Arc::new(Mutex::new(clear_rx)),
            handler: ClearEndpoint::new(clear_tx),
        };

        let shapes_reciever = EndpointReciever {
            reciever: Arc::new(Mutex::new(shapes_rx)),
            handler: ShapesEndpoint::new(shapes_tx),
        };

        let multi_shapes_reciever = EndpointReciever {
            reciever: Arc::new(Mutex::new(multi_shapes_rx)),
            handler: MultiShapesEndpoint::new(multi_shapes_tx),
        };

        EndpointRecieverStack {
            clear_reciever,
            shapes_reciever,
            multi_shapes_reciever,
        }
    }

    pub fn to_recievers(
        &self,
    ) -> (
        Arc<Mutex<Receiver<ClearPayload>>>,
        Arc<Mutex<Receiver<ShapePayload>>>,
        Arc<Mutex<Receiver<MultipleShapesPayload>>>,
    ) {
        // let somevar = self.shapes_reciever.handler.handle();

        return (
            self.clear_reciever.reciever.clone(),
            self.shapes_reciever.reciever.clone(),
            self.multi_shapes_reciever.reciever.clone(),
        );
    }
}
