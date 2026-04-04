use openssl::error::ErrorStack;
use serde::{Deserialize, Serialize};

use crate::{
    beacon::Beacon,
    blockchain::{
        block::{Block, genesis_block},
        transaction::Transaction,
    },
    util::key::SK,
};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
struct Chain {
    blocks: Vec<Block>,
}

impl Chain {
    pub fn new() -> Self {
        Self {
            blocks: vec![genesis_block()],
        }
    }

    pub fn get_latest_block(&self) -> Block {
        match self.blocks.last() {
            Some(block) => block.clone(),
            None => genesis_block(),
        }
    }

    pub fn generate_next_block(
        &self,
        sk: &SK,
        beacon: Beacon,
        transactions: Vec<Transaction>,
    ) -> Result<Block, ErrorStack> {
        let previous_block: Block = self.get_latest_block();
        let next_index: u64 = previous_block.index + 1;
        let next_timestamp: i64 = chrono::Utc::now().timestamp_millis();
        Block::new_with_creating_signature(
            next_index,
            next_timestamp,
            transactions,
            beacon,
            vec![],
            &previous_block.issuer,
            previous_block.hash,
            sk,
        )
    }
}

pub fn is_valid_new_block(block: &Block, previous_block: &Block) -> bool {
    block.index == previous_block.index + 1
        && block.timestamp > previous_block.timestamp
        && block.previous_hash == previous_block.hash
        && block.calculate_hash() == block.hash
}
