use crate::{
    blockchain::{address::Address, transaction::Transaction},
    state::State,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Event {
    AddTransaction(Address, u64),
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Effect {
    None,
}

pub fn update(event: Event, state: State) -> (State, Vec<Effect>) {
    match event {
        Event::AddTransaction(address, amount) => {
            if let Ok(transaction) = Transaction::new_with_creating_signature(
                &state.address,
                &address,
                amount,
                &state.secret_key,
            ) {
                return (
                    State {
                        transactions: state
                            .transactions
                            .into_iter()
                            .chain([transaction])
                            .collect(),
                        ..state
                    },
                    Vec::new(),
                );
            };
        }
    }
    (state, Vec::new())
}

pub async fn run_effect(effect: Effect) {}
