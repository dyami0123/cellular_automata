use crate::server::endpoint_handler::EndpointHandler;
use crate::server::payloads::Payload;
use axum::Router;
pub struct ApiRouter {
    route_stack: Vec<(String, axum::routing::MethodRouter)>,
}

impl ApiRouter {
    pub fn new() -> Self {
        Self {
            route_stack: Vec::new(),
        }
    }

    pub fn register_endpoint<H, T>(&mut self, path: String, route: axum::routing::MethodRouter)
    where
        H: EndpointHandler<T> + Send + Sync + 'static,
        T: Payload,
    {
        self.route_stack.push((path, route));
    }

    fn get_router(self) -> Router {
        let mut router = Router::new();
        for (path, route) in self.route_stack {
            router = router.route(&path, route);
        }
        router
    }

    pub fn spawn(self) -> tokio::task::JoinHandle<()> {
        tokio::spawn(async move {
            let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
                .await
                .unwrap();
            tracing::warn!("listening on {}", listener.local_addr().unwrap());
            axum::serve(listener, self.get_router()).await.unwrap();
        })
    }
}
