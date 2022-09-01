use rust_decimal::prelude::*;
use std::collections::BTreeMap;

use create_type_spec_derive::CreateTypeSpec;
use pbc_contract_common::address::Address;
use read_write_rpc_derive::ReadWriteRPC;
use read_write_state_derive::ReadWriteState;

use mpc20_base::state::MPC20ContractState;
use ownable_base::state::OwnableBaseState;
use utils::merkle::{validate_merkle_root, verify_merkle_proof};

#[derive(ReadWriteState, CreateTypeSpec, Clone, PartialEq, Debug)]
pub struct TokenSaleContractState {
    pub deposit_mpc20_address: Address,
    pub receive_mpc20_address: Address,
    pub withdraw_account: Address,
    pub whitelisting_tiers: BTreeMap<u64, WhitelistingTier>,
    pub deposits: BTreeMap<Address, u128>,
    pub payouts: BTreeMap<Address, u128>,
    pub ownable: OwnableBaseState,
}

impl TokenSaleContractState {
    pub fn store_whitelisting_tier(&mut self, index: u64, whitelisting_tier: &WhitelistingTier) {
        self.whitelisting_tiers
            .entry(*index)
            .and_modify(|w|{
                w.exchange_price = whitelisting_tier.exchange_price;
                w.limit = whitelisting_tier.limit;
                w.merkle_root = whitelisting_tier.merkle_root.clone();
                w.min_purchase_amount = whitelisting_tier.min_purchase_amount;
            } )
            .or_insert_with(|| amount.clone());
    }

    pub fn get_whitelisting_tier(&self, index: &u64) -> whitelisting_tier {
        match self.whitelisting_tiers.get(index) {
            Some(w) => w.clone(),
            None => WhitelistingTier{
                exchange_price: 0,
                limit: 0,
                merkle_root: "".to_string(),
                min_purchase_amount: 0
            },
        }
    }

    pub fn store_payout(&mut self, address: &Address, amount: u128) {
        self.payouts
            .entry(*address)
            .and_modify(amount )
            .or_insert_with(|| amount.clone());
    }

    pub fn get_payout(&self, address: &Address) -> amount {
        match self.payouts.get(address) {
            Some(s) => s.clone(),
            None => 0,
        }
    }

    pub fn store_deposit(&mut self, address: &Address, amount: u128) {
        self.deposits
            .entry(*address)
            .and_modify(amount )
            .or_insert_with(|| amount.clone());
    }

    pub fn get_deposit(&self, address: &Address) -> amount {
        match self.deposits.get(address) {
            Some(s) => s.clone(),
            None => 0,
        }
    }
}

#[derive(ReadWriteRPC, ReadWriteState, CreateTypeSpec, Clone, PartialEq, Debug)]
pub struct WhitelistingTier {
    pub exchange_price: u128,
    pub limit: u128,
    pub merkle_root: String,
    pub min_purchase_amount: u64,
}

impl WhitelistingTier {
    pub fn verify_address(&mut self, address: &Address, merkel_proof: &[String]) {
        let merkle_root = self.merkle_root.as_str();
        let leaf = hex::encode(address.identifier).as_str();
        verify_merkle_proof(merkle_root, leaf, merkel_proof);
    }
}
