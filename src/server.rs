use axum::{extract::State, routing::get, Json, Router};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing::info;

use std::{io, sync::Arc};

use crate::queue::Queue;

#[derive(Clone)]
pub struct AppState {
    queue: Arc<Queue>,
}

pub struct Server {
    queue: Arc<Queue>,
}

impl Server {
    pub fn new(queue: Arc<Queue>) -> Self {
        Self { queue }
    }

    pub async fn start(&self) -> Result<(), io::Error> {
        let state = Arc::new(AppState {
            queue: self.queue.clone(),
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
