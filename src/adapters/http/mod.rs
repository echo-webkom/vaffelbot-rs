use axum::{extract::State, routing::get, Json, Router};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing::info;

use std::{io, sync::Arc};

use crate::domain::{OrderRepository, QueueRepository};

#[derive(Clone)]
pub struct AppState {
    queue: Arc<dyn QueueRepository>,
    #[allow(dead_code)]
    orders: Arc<dyn OrderRepository>,
}

pub struct HttpAdapter {
    queue: Arc<dyn QueueRepository>,
    orders: Arc<dyn OrderRepository>,
}

impl HttpAdapter {
    pub fn new(queue: Arc<dyn QueueRepository>, orders: Arc<dyn OrderRepository>) -> Self {
        Self { queue, orders }
    }

    pub async fn start(&self) -> Result<(), io::Error> {
        let state = Arc::new(AppState {
            queue: self.queue.clone(),
            orders: self.orders.clone(),
        });

        let app = Router::new()
            .route("/queue", get(list_queue))
            .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()))
            .with_state(state);

        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
        info!("listening on http://{}", listener.local_addr().unwrap());

        axum::serve(listener, app).await
    }
}

async fn list_queue(State(state): State<Arc<AppState>>) -> Json<Vec<String>> {
    let queue = state.queue.list();
    Json(queue)
}
