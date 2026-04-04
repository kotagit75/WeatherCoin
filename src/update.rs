use crate::state::State;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Event {
    None,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Effect {
    None,
}

pub fn update(event: Event, state: State) -> (State, Vec<Effect>) {
    (state, vec![Effect::None])
}

pub async fn run_effect(effect: Effect) {}
