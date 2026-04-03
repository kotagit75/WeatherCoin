use serde::{Deserialize, Serialize};

use crate::{address::Address, util::signature::Signature};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Transaction {
    pub sender: Address,
    pub recipient: Address,
    pub amount: u64,
    pub signature: Signature,
}

impl std::fmt::Display for Transaction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} -> {}: {}", self.sender, self.recipient, self.amount)?;
        Ok(())
    }
}
