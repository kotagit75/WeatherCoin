use tokio::sync::mpsc;

use crate::update::{run_effect, update};

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
    while let Some((new_state, effects)) = event_rx.recv().await.map(|event| update(event, state)) {
        state = new_state;
        for effect in effects {
            run_effect(effect).await;
        }
    }
}
