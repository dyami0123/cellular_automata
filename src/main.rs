pub mod ggez_backend;
pub mod rendering;
pub mod server;
pub mod shapes;

#[tokio::main]
async fn main() {
    server::server::run().await;
}
