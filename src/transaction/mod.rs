use hashbrown::HashMap;
use serde::Deserialize;

use crate::utils::arbitrary_tx_amount;

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[repr(u8)]
pub enum TransactionKind {
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
    Chargeback,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct TransactionData {
    #[serde(rename = "type")]
    pub kind: TransactionKind,

    #[serde(default, deserialize_with = "arbitrary_tx_amount")]
    pub amount: f32,
}


#[derive(Deserialize, Debug)]
pub struct Transaction {
    #[serde(rename = "client")]
    pub client_id: u16,

    #[serde(rename = "tx")]
    pub id: u32,

    #[serde(flatten)]
    transaction_data: TransactionData,
}

impl Transaction {
    pub fn get_data(&self) -> TransactionData {
        self.transaction_data.clone()
    }
}

pub type Transactions = HashMap<u32, TransactionData>;
