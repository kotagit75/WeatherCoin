use tokio::sync::{mpsc, watch};

use crate::update::{run_effect, update};

pub mod api;
pub mod beacon;
pub mod blockchain;
pub mod node;
pub mod state;
pub mod update;
pub mod util;

#[tokio::main]
async fn main() {
    let Ok(sk) = node::load_key() else {
        return;
    };
    let Ok(mut state) = state::State::new(sk) else {
        return;
    };
    println!("address: {:?}", state.address);

    let (event_tx, mut event_rx) = mpsc::channel(256);
    let (state_tx, mut state_rx) = watch::channel(state.clone());
    let api_tx = event_tx.clone();
    tokio::spawn(async move {
        api::init_api(api_tx, state_rx).await;
    });
    while let Some((new_state, effects)) = event_rx.recv().await.map(|event| update(event, state)) {
        state = new_state;
        let _ = state_tx.send(state.clone());
        for effect in effects {
            run_effect(effect).await;
        }
    }
}
