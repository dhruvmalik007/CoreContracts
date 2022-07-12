use std::collections::BTreeMap;

use create_type_spec_derive::CreateTypeSpec;
use pbc_contract_common::address::Address;
use read_write_rpc_derive::ReadWriteRPC;
use read_write_state_derive::ReadWriteState;

use crate::ContractError;

#[state]
#[derive(PartialEq, Debug)]
pub struct MPC721ContractState {
    pub owner: Option<Address>,
    pub name: String,
    pub symbol: String,
    pub base_uri: Option<String>,
    pub minter: Address,
    pub supply: u128,
    pub tokens: BTreeMap<u128, TokenInfo>,
    pub operator_approvals: BTreeMap<Address, BTreeMap<Address, bool>>,
}

#[derive(ReadWriteRPC, ReadWriteState, CreateTypeSpec, Clone, PartialEq, Debug)]
pub struct TokenInfo {
    pub owner: Address,
    pub approvals: Vec<Address>,
    pub token_uri: Option<String>,
}

impl MPC721ContractState {
    pub fn set_base_uri(&mut self, base_uri: &str) {
        self.base_uri = Some(base_uri.to_string())
    }

    pub fn mint(&mut self, token_id: u128, to: &Address, token_uri: &Option<String>) {
        let token = TokenInfo {
            owner: *to,
            approvals: vec![],
            token_uri: token_uri.clone(),
        };

        self.tokens.insert(token_id, token);
    }

    pub fn increase_supply(&mut self) {
        self.supply = self.supply.checked_add(1).unwrap()
    }

    pub fn decrease_supply(&mut self) {
        self.supply = self.supply.checked_sub(1).unwrap()
    }

    pub fn transfer(&mut self, from: &Address, to: &Address, token_id: u128) {
        let token = self.tokens.get(&token_id).unwrap();
        assert!(
            Self::allowed_to_transfer(from, token, &self.operator_approvals),
            "{}",
            ContractError::Unauthorized
        );

        self.tokens.entry(token_id).and_modify(|t| {
            t.owner = *to;
            t.approvals = vec![];
        });
    }

    pub fn update_approvals(
        &mut self,
        from: &Address,
        spender: &Address,
        token_id: u128,
        approved: bool,
    ) {
        let token = self.tokens.get(&token_id).unwrap().to_owned();
        assert!(
            Self::allowed_to_approve(from, &token, &self.operator_approvals),
            "{}",
            ContractError::Unauthorized,
        );

        let mut approvals = token
            .approvals
            .into_iter()
            .filter(|account| account != spender)
            .collect::<Vec<Address>>();

        if approved {
            approvals.push(*spender);
        }

        self.tokens
            .entry(token_id)
            .and_modify(|t| t.approvals = approvals);
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

    pub fn remove_token(&mut self, owner: &Address, token_id: u128) {
        let token = self.tokens.get(&token_id).unwrap();
        assert!(
            Self::allowed_to_transfer(owner, token, &self.operator_approvals),
            "{}",
            ContractError::Unauthorized
        );

        self.tokens.remove(&token_id);
    }

    pub fn is_minted(&self, token_id: u128) -> bool {
        self.tokens.contains_key(&token_id)
    }

    pub fn is_owner(&self, address: &Address) -> bool {
        if let Some(owner) = self.owner {
            owner.eq(address)
        } else {
            false
        }
    }

    pub fn token_info(&self, token_id: u128) -> Option<&TokenInfo> {
        self.tokens.get(&token_id)
    }

    pub fn balance_of(&self, owner: &Address) -> u128 {
        self.tokens
            .values()
            .into_iter()
            .filter(|ti| ti.owner == *owner)
            .count() as u128
    }

    pub fn owner_of(&self, token_id: u128) -> Address {
        self.tokens.get(&token_id).unwrap().owner
    }

    fn allowed_to_transfer(
        account: &Address,
        token: &TokenInfo,
        operator_approvals: &BTreeMap<Address, BTreeMap<Address, bool>>,
    ) -> bool {
        if token.owner == *account {
            return true;
        }

        if token.approvals.iter().any(|spender| spender == account) {
            return true;
        }

        if let Some(owner_approvals) = operator_approvals.get(&token.owner) {
            if let Some(approved) = owner_approvals.get(account) {
                return *approved;
            }
        }

        false
    }

    fn allowed_to_approve(
        account: &Address,
        token: &TokenInfo,
        operator_approvals: &BTreeMap<Address, BTreeMap<Address, bool>>,
    ) -> bool {
        if token.owner == *account {
            return true;
        }

        if let Some(owner_approvals) = operator_approvals.get(&token.owner) {
            if let Some(approved) = owner_approvals.get(account) {
                return *approved;
            }
        }

        false
    }
}
