use crate::server::payloads::Payload;
use tokio::sync::mpsc;
pub trait EndpointHandler<T: Payload>: Send + Sync {
    fn sender(&self) -> &mpsc::Sender<T>;

    // fn handler_function(&self) -> impl Future<Output = ()>;

    fn new(sender: mpsc::Sender<T>) -> Self;
}
