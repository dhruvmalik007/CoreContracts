use std::collections::BTreeMap;

use create_type_spec_derive::CreateTypeSpec;
use pbc_contract_common::address::Address;
use read_write_rpc_derive::ReadWriteRPC;
use read_write_state_derive::ReadWriteState;

use crate::ContractError;

#[derive(ReadWriteState, CreateTypeSpec, Clone, PartialEq, Debug)]
pub struct MPC1155ContractState {
    pub owner: Option<Address>,
    pub uri: String,
    pub minter: Address,
    pub balances: BTreeMap<u128, BTreeMap<Address, u128>>,
    pub operator_approvals: BTreeMap<Address, BTreeMap<Address, bool>>,
    pub tokens: BTreeMap<u128, TokenInfo>,
}

#[derive(ReadWriteRPC, ReadWriteState, CreateTypeSpec, Clone, PartialEq, Debug)]
pub struct TokenInfo {
    pub token_uri: Option<String>,
}

impl MPC1155ContractState {
    pub fn set_uri(&mut self, uri: &str) {
        self.uri = uri.to_string()
    }

    pub fn store_token(&mut self, token_id: u128, info: &TokenInfo) {
        self.tokens.entry(token_id).or_insert_with(|| info.clone());
    }

    pub fn transfer(
        &mut self,
        from: Option<&Address>,
        to: Option<&Address>,
        token_id: u128,
        amount: u128,
    ) {
        if let Some(from) = from {
            self.balances.entry(token_id).and_modify(|token_balances| {
                token_balances
                    .entry(*from)
                    .and_modify(|balance| *balance = balance.checked_sub(amount).unwrap());
            });
        }

        if let Some(to) = to {
            self.balances
                .entry(token_id)
                .and_modify(|token_balances| {
                    token_balances
                        .entry(*to)
                        .and_modify(|balance| *balance = balance.checked_add(amount).unwrap())
                        .or_insert(amount);
                })
                .or_insert_with(|| BTreeMap::from([(*to, amount)]));
        }
    }

    pub fn add_operator(&mut self, owner: &Address, operator: &Address) {
        let owner_operators = self
            .operator_approvals
            .entry(*owner)
            .or_insert_with(BTreeMap::new);

        owner_operators.insert(*operator, true);
    }

    pub fn remove_operator(&mut self, owner: &Address, operator: &Address) {
        let owner_operators = self
            .operator_approvals
            .get_mut(owner)
            .unwrap_or_else(|| panic!("{}", ContractError::NotFound.to_string()));

        owner_operators.remove(operator);

        if owner_operators.is_empty() {
            self.operator_approvals.remove(owner);
        }
    }

    pub fn is_owner(&self, address: &Address) -> bool {
        if let Some(owner) = self.owner {
            owner.eq(address)
        } else {
            false
        }
    }

    pub fn is_token_owner_or_operator(&self, owner: &Address, sender: &Address) -> bool {
        if owner == sender {
            return true;
        }

        if let Some(owner_approvals) = self.operator_approvals.get(owner) {
            if let Some(approved) = owner_approvals.get(sender) {
                return *approved;
            }
        }

        false
    }

    pub fn token_info(&self, token_id: u128) -> Option<&TokenInfo> {
        self.tokens.get(&token_id)
    }
}
