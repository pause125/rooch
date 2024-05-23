// Copyright (c) RoochNetwork
// SPDX-License-Identifier: Apache-2.0

use crate::schema::transactions;
use crate::types::IndexedTransaction;
use diesel::prelude::*;
use move_core_types::vm_status::KeptVMStatus;
use moveos_types::h256::H256;
use moveos_types::transaction::TransactionExecutionInfo;
use rooch_types::transaction::{LedgerTransaction, TransactionWithInfo};
use std::str::FromStr;

#[derive(Clone, Debug, Queryable, Insertable, QueryableByName)]
#[diesel(table_name = transactions)]
pub struct StoredTransaction {
    /// The hash of this transaction.
    // pub tx_hash: Varchar(65),
    #[diesel(sql_type = diesel::sql_types::Text)]
    pub tx_hash: String,
    /// The tx order of this transaction.
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    pub tx_order: i64,

    #[diesel(sql_type = diesel::sql_types::BigInt)]
    pub sequence_number: i64,
    /// the rooch address of sender who send the transaction
    #[diesel(sql_type = diesel::sql_types::Text)]
    pub sender: String,
    #[diesel(sql_type = diesel::sql_types::Text)]
    pub action: String,
    #[diesel(sql_type = diesel::sql_types::SmallInt)]
    pub action_type: i16,
    #[diesel(sql_type = diesel::sql_types::Blob)]
    pub action_raw: Vec<u8>,
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    pub auth_validator_id: i64,
    #[diesel(sql_type = diesel::sql_types::Blob)]
    pub authenticator_payload: Vec<u8>,
    #[diesel(sql_type = diesel::sql_types::Text)]
    pub tx_accumulator_root: String,
    #[diesel(sql_type = diesel::sql_types::Blob)]
    pub transaction_raw: Vec<u8>,

    #[diesel(sql_type = diesel::sql_types::Text)]
    pub state_root: String,
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    pub size: i64,
    #[diesel(sql_type = diesel::sql_types::Text)]
    pub event_root: String,
    /// The amount of gas used.
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    pub gas_used: i64,
    /// The vm status.
    #[diesel(sql_type = diesel::sql_types::Text)]
    pub status: String,

    #[diesel(sql_type = diesel::sql_types::BigInt)]
    pub created_at: i64,
}

impl From<IndexedTransaction> for StoredTransaction {
    fn from(transaction: IndexedTransaction) -> Self {
        StoredTransaction {
            tx_hash: format!("{:?}", transaction.tx_hash),
            tx_order: transaction.tx_order as i64,
            sequence_number: transaction.sequence_number as i64,
            sender: transaction.sender.to_hex_literal(),
            action: transaction.action.action_name(),
            action_type: transaction.action.action_type() as i16,
            action_raw: transaction.action_raw,
            auth_validator_id: transaction.auth_validator_id as i64,
            authenticator_payload: transaction.authenticator_payload,
            tx_accumulator_root: format!("{:?}", transaction.tx_accumulator_root),
            transaction_raw: transaction.transaction_raw,
            state_root: format!("{:?}", transaction.state_root),
            size: transaction.size as i64,
            event_root: format!("{:?}", transaction.event_root),
            gas_used: transaction.gas_used as i64,
            status: transaction.status,
            created_at: transaction.created_at as i64,
        }
    }
}

impl StoredTransaction {
    pub fn try_into_transaction_with_info(self) -> Result<TransactionWithInfo, anyhow::Error> {
        let transaction = LedgerTransaction::decode(&self.transaction_raw)?;

        let status: KeptVMStatus = serde_json::from_str(self.status.as_str())?;
        let execution_info = TransactionExecutionInfo {
            tx_hash: H256::from_str(self.tx_hash.as_str())?,
            state_root: H256::from_str(self.state_root.as_str())?,
            size: self.size as u64,
            event_root: H256::from_str(self.event_root.as_str())?,
            gas_used: self.gas_used as u64,
            status,
        };
        Ok(TransactionWithInfo {
            transaction,
            execution_info,
        })
    }
}
