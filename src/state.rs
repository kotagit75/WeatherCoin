use openssl::error::ErrorStack;
use serde::{Deserialize, Serialize};

use crate::{
    blockchain::{address::Address, chain::Chain},
    util::key::SK,
};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct State {
    pub secret_key: SK,
    pub address: Address,
    pub chain: Chain,
}

impl State {
    pub fn new(secret_key: SK) -> Result<Self, ErrorStack> {
        secret_key.to_pk().map(|address| Self {
            secret_key,
            address,
            chain: Chain::new(),
        })
    }
}
