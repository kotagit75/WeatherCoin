use openssl::error::ErrorStack;
use serde::{Deserialize, Serialize};

use crate::{
    blockchain::address::Address,
    util::{key::SK, signature::Signature},
};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct TransactionOut {
    pub address: Address,
    pub amount: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Transaction {
    pub sender: Address,
    pub out: Vec<TransactionOut>,
    pub un_spent_id: u64,
    pub signature: Signature,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct UnspentTransaction {
    pub id: u64,
    pub address: Address,
    pub amount: u64,
}

impl std::fmt::Display for TransactionOut {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}: {}", self.address, self.amount)?;
        Ok(())
    }
}
impl std::fmt::Display for Transaction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{} -> {}: {}",
            self.sender,
            self.out
                .iter()
                .map(|txout| txout.address.der.clone())
                .collect::<Vec<_>>()
                .join(", "),
            self.total_amount()
        )?;
        Ok(())
    }
}

impl TransactionOut {
    pub fn to_unspent(&self, id: u64) -> (UnspentTransaction, u64 /*new id */) {
        (
            UnspentTransaction {
                id,
                address: self.address.clone(),
                amount: self.amount,
            },
            id + 1,
        )
    }
}

impl Transaction {
    pub fn new(
        sender: Address,
        out: Vec<TransactionOut>,
        un_spent_id: u64,
        signature: Signature,
    ) -> Self {
        Self {
            sender,
            out,
            un_spent_id,
            signature,
        }
    }
    pub fn new_with_creating_signature(
        sender: &Address,
        out: Vec<TransactionOut>,
        un_spent_id: u64,
        sk: &SK,
    ) -> Result<Self, ErrorStack> {
        Ok(Self {
            sender: sender.clone(),
            out: out.clone(),
            un_spent_id,
            signature: create_transaction_signature(sender, &out, un_spent_id, sk)?,
        })
    }
    pub fn verify_signature(&self) -> bool {
        self.sender.clone().verify(
            transacction_to_buf_for_signature(&self.sender, &self.out, self.un_spent_id).as_slice(),
            &self.signature,
        )
    }

    pub fn total_amount(&self) -> u64 {
        self.out.iter().map(|txout| txout.amount).sum()
    }

    pub fn get_unspent_transactions(
        &self,
        (previous_unspent, first_id): (Vec<UnspentTransaction>, u64),
    ) -> (Vec<UnspentTransaction>, u64 /*new id */) {
        let (mut new_unspent, new_id) =
            self.out
                .iter()
                .fold((previous_unspent, first_id), |(mut acc, id), tx_out| {
                    let (unspent, new_id) = tx_out.to_unspent(id);
                    acc.push(unspent);
                    (acc, new_id)
                });
        new_unspent.retain(|unspent| unspent.id != self.un_spent_id);
        (new_unspent, new_id)
    }

    pub fn is_valid(&self) -> bool {
        self.verify_signature() && self.total_amount() > 0
    }
}

fn transacction_to_buf_for_signature(
    sender: &Address,
    out: &[TransactionOut],
    un_spent_id: u64,
) -> Vec<u8> {
    format!("{sender}{out:?}{un_spent_id}").as_bytes().to_vec()
}

fn create_transaction_signature(
    sender: &Address,
    out: &[TransactionOut],
    un_spent_id: u64,
    sk: &SK,
) -> Result<Signature, ErrorStack> {
    let data = transacction_to_buf_for_signature(sender, out, un_spent_id);
    sk.sign(&data)
}

pub fn get_transaction_out(
    sender: &Address,
    recipient: &Address,
    amount: u64,
    unspent_amount: u64,
) -> Vec<TransactionOut> {
    vec![
        TransactionOut {
            address: recipient.clone(),
            amount,
        },
        TransactionOut {
            address: sender.clone(),
            amount: unspent_amount - amount,
        },
    ]
}

const COINBASE_AMOUNT: u64 = 50;
fn coinbase_address() -> Address {
    Address {
        der: "".to_string(),
    }
}
pub fn coinbase_transaction(address: &Address) -> Transaction {
    Transaction {
        sender: coinbase_address(),
        out: vec![TransactionOut {
            address: address.clone(),
            amount: COINBASE_AMOUNT,
        }],
        un_spent_id: 0,
        signature: Signature::default(),
    }
}

pub fn is_valid_coinbase_transaction(transaction: &Transaction) -> bool {
    transaction.sender == coinbase_address()
        && transaction.un_spent_id == 0
        && transaction.out.len() == 1
        && transaction.out[0].amount == COINBASE_AMOUNT
}
