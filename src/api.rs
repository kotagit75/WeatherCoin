use axum::{Router, extract, response, routing::get};
use tokio::sync::{mpsc, watch};

use crate::{state::State, update::Event};

const API_PORT: u32 = 8080;
pub async fn init_api(event_tx: mpsc::Sender<Event>, state_rx: watch::Receiver<State>) {
    let app = Router::new()
        .route("/state", get(handle_get_state))
        .with_state((event_tx, state_rx));
    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", API_PORT))
        .await
        .unwrap();
    println!("API server is running on http://localhost:{}", API_PORT);
    axum::serve(listener, app).await.unwrap();
}

async fn handle_get_state(
    extract::State((_, mut state_rx)): extract::State<(
        mpsc::Sender<Event>,
        watch::Receiver<State>,
    )>,
) -> response::Json<State> {
    response::Json(state_rx.borrow().clone())
}
