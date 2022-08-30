use hashbrown::HashMap;
use tinyset::SetU32;

use crate::{
    balance::Balance,
    transaction::{TransactionData, TransactionKind, Transactions},
    types::Result,
};

#[derive(Debug, Default, PartialEq)]
pub struct Client {

    balance: Balance,

    transactions: Transactions,

    disputed_transactions: SetU32,

    previous_tx_id: u32,

    locked: bool,
}

impl Client {
    pub fn new(tx_id: u32, tx_data: TransactionData) -> Self {
        let balance = if tx_data.kind == TransactionKind::Deposit {
            Balance::new(tx_data.amount)
        } else {
            Balance::default()
        };

        let mut client = Client {
            balance,
            transactions: Transactions::new(),
            disputed_transactions: SetU32::new(),
            previous_tx_id: tx_id,
            locked: false,
        };

        client.record_tx(tx_id, tx_data);
        client
    }

    #[inline]
    fn record_tx(&mut self, tx_id: u32, tx_data: TransactionData) {
        self.transactions.insert(tx_id, tx_data);
        self.previous_tx_id = tx_id;
    }

   
    #[inline]
    fn get_tx(&self, tx_id: u32) -> Result<&TransactionData> {
        match self.transactions.get(&tx_id) {
            Some(tx) => Ok(tx),
            None => Err(format!(
                "Can't retrieve transaction with ID {tx_id}! \
                Either it was for a different user or we haven't seen it at all."
            )
            .into()),
        }
    }

   
    #[inline]
    fn check_tx_id(&self, tx_id: u32) -> Result<()> {
        if self.previous_tx_id < tx_id {
            Ok(())
        } else {
            Err(format!(
                "Newly seen transactions should have a higher ID! \
                Previously seen transaction ID: {}. \
                Current one: {tx_id}.",
                self.previous_tx_id
            )
            .into())
        }
    }

    #[inline]
    fn amount_is_available(&self, tx_amount: f32) -> Result<()> {
        if self.balance.available >= tx_amount {
            Ok(())
        } else {
            return Err(format!(
                "The client doesn't have sufficient funds! \
                Funds available: {}. \
                Transaction amount: {tx_amount}.",
                self.balance.available
            )
            .into());
        }
    }

   
    #[inline]
    fn transaction_is_disputed(&self, tx_id: u32, expected: bool) -> Result<()> {
        if self.disputed_transactions.contains(tx_id) != expected {
            Err(format!(
                "The transaction with ID {tx_id} is{}disputed! The opposite was expected.",
                if expected { " not " } else { " " }
            )
            .into())
        } else {
            Ok(())
        }
    }

    
    #[inline]
    fn account_is_locked(&self, tx_id: u32) -> Result<()> {
        if self.locked {
            Err(format!("Transaction to the locked account! Transaction with ID {tx_id} was ignored.").into())
        } else {
            Ok(())
        }
    }

    
    pub fn process_tx(&mut self, tx_id: u32, tx_data: TransactionData) -> Result<()> {
        self.account_is_locked(tx_id)?;

        match tx_data.kind {
            TransactionKind::Deposit => {
                self.check_tx_id(tx_id)?;
                self.balance.available += tx_data.amount;
                self.record_tx(tx_id, tx_data);
            },
            TransactionKind::Withdrawal => {
                self.check_tx_id(tx_id)?;
                self.amount_is_available(tx_data.amount)?;
                self.balance.available -= tx_data.amount;
                self.record_tx(tx_id, tx_data);
            },
            TransactionKind::Dispute => {
                self.transaction_is_disputed(tx_id, false)?;
                let disputed = self.get_tx(tx_id)?.amount;

                self.amount_is_available(disputed)?;

                self.balance.available -= disputed;
                self.balance.held += disputed;
                self.disputed_transactions.insert(tx_id);
            },
            TransactionKind::Resolve => {
                self.transaction_is_disputed(tx_id, true)?;
                let disputed = self.get_tx(tx_id)?.amount;
                if self.balance.held >= disputed {
                    self.balance.held -= disputed;
                    self.balance.available += disputed;
                    self.disputed_transactions.remove(tx_id);
                }
            },
            TransactionKind::Chargeback => {
                self.transaction_is_disputed(tx_id, true)?;
                let disputed = self.get_tx(tx_id)?.amount;

                if self.balance.held >= disputed {
                    self.balance.held -= disputed;
                    self.locked = true;
                    self.disputed_transactions.remove(tx_id);
                }
            },
        }

        Ok(())
    }

    
    pub fn get_record(&self, client_id: u16) -> Vec<String> {
        vec![
            client_id.to_string(),
            format!("{:.4}", self.balance.available),
            format!("{:.4}", self.balance.held),
            format!("{:.4}", self.balance.available + self.balance.held),
            self.locked.to_string(),
        ]
    }
}

pub type Clients = HashMap<u16, Client>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_new_client_from_deposit() {
        let tx_data = TransactionData {
            kind: TransactionKind::Deposit,
            amount: 56.8953,
        };
        let client = Client::new(78, tx_data.clone());
        let mut expected_client = Client {
            balance: Balance {
                available: 56.8953,
                held: 0.0,
            },
            transactions: Transactions::new(),
            disputed_transactions: SetU32::new(),
            previous_tx_id: 78,
            locked: false,
        };
        expected_client.record_tx(78, tx_data);
        assert_eq!(client, expected_client);
    }

    #[test]
    fn should_create_new_client_from_arbitrary_tx() {
        let tx_data = TransactionData {
            kind: TransactionKind::Dispute,
            amount: 657.5675,
        };
        let client = Client::new(3438, tx_data.clone());
        let mut expected_client = Client {
            balance: Balance {
                available: 0.0,
                held: 0.0,
            },
            transactions: Transactions::new(),
            disputed_transactions: SetU32::new(),
            previous_tx_id: 0,
            locked: false,
        };
        expected_client.record_tx(3438, tx_data);
        assert_eq!(client, expected_client);
    }
}
