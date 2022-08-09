use std::collections::BTreeMap;

use create_type_spec_derive::CreateTypeSpec;
use pbc_contract_common::address::Address;
use read_write_rpc_derive::ReadWriteRPC;
use read_write_state_derive::ReadWriteState;

use crate::{msg::InitialBalance, ContractError};

#[derive(ReadWriteState, CreateTypeSpec, Clone, PartialEq, Debug)]
pub struct MPC20ContractState {
    pub info: TokenInfo,
    pub total_supply: u128,
    pub minter: Option<Minter>,
    pub balances: BTreeMap<Address, u128>,
    pub allowances: BTreeMap<Address, BTreeMap<Address, u128>>,
}

#[derive(ReadWriteRPC, ReadWriteState, CreateTypeSpec, Clone, PartialEq, Debug)]
pub struct TokenInfo {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
}

#[derive(ReadWriteRPC, ReadWriteState, CreateTypeSpec, Clone, PartialEq, Debug)]
pub struct Minter {
    pub minter: Address,
    pub capacity: Option<u128>,
}

impl MPC20ContractState {
    pub fn new(info: &TokenInfo, minter: &Option<Minter>) -> Self {
        Self {
            info: info.clone(),
            total_supply: 0,
            minter: minter.clone(),
            balances: BTreeMap::new(),
            allowances: BTreeMap::new(),
        }
    }

    pub fn init_balances(&mut self, initial_balances: &[InitialBalance]) -> u128 {
        let mut total_supply: u128 = 0;
        for ib in initial_balances {
            self.balances.insert(ib.address, ib.amount);
            total_supply += ib.amount;
        }

        self.total_supply = total_supply;
        total_supply
    }

    pub fn mint_to(&mut self, to: &Address, amount: u128) {
        self.increase_total_supply(amount);
        if let Some(limit) = self.get_capacity() {
            assert!(
                self.total_supply <= limit,
                "{}",
                ContractError::CapacityExceeded
            );
        }

        self.increase_balance(to, amount);
    }

    pub fn increase_balance(&mut self, address: &Address, amount: u128) {
        Self::increase_or_set(&mut self.balances, address, amount);
    }

    pub fn decrease_balance(&mut self, address: &Address, amount: u128) {
        Self::decrease_or_remove(&mut self.balances, address, amount);
    }

    pub fn increase_total_supply(&mut self, amount: u128) {
        self.total_supply += amount
    }

    pub fn decrease_total_supply(&mut self, amount: u128) {
        self.total_supply = self.total_supply.checked_sub(amount).unwrap();
    }

    pub fn set_allowance(&mut self, owner: &Address, spender: &Address, amount: u128) {
        let owner_allowances = self.allowances.entry(*owner).or_insert_with(BTreeMap::new);
        owner_allowances.insert(*spender, amount);
    }

    pub fn increase_allowance(&mut self, owner: &Address, spender: &Address, amount: u128) {
        let owner_allowances = self.allowances.entry(*owner).or_insert_with(BTreeMap::new);
        Self::increase_or_set(owner_allowances, spender, amount);
    }

    pub fn decrease_allowance(&mut self, owner: &Address, spender: &Address, amount: u128) {
        let owner_allowances = self
            .allowances
            .get_mut(owner)
            .unwrap_or_else(|| panic!("{}", ContractError::NotFound.to_string()));

        Self::decrease_or_remove(owner_allowances, spender, amount);
    }

    fn increase_or_set(map: &mut BTreeMap<Address, u128>, address: &Address, amount: u128) {
        map.entry(*address)
            .and_modify(|a| *a += amount)
            .or_insert(amount);
    }

    fn decrease_or_remove(map: &mut BTreeMap<Address, u128>, address: &Address, amount: u128) {
        let current = *map
            .get(address)
            .unwrap_or_else(|| panic!("{}", ContractError::NotFound.to_string()));

        assert!(current >= amount, "{}", ContractError::Overflow.to_string());

        if amount < current {
            map.entry(*address).and_modify(|a| *a -= amount);
        } else {
            map.remove(address);
        }
    }

    pub fn get_capacity(&self) -> Option<u128> {
        self.minter.as_ref().and_then(|m| m.capacity)
    }

    pub fn balance_of(&self, address: &Address) -> u128 {
        *self.balances.get(address).unwrap_or(&0)
    }

    pub fn allowance(&self, owner: &Address, spender: &Address) -> u128 {
        *self
            .allowances
            .get(owner)
            .unwrap_or(&BTreeMap::new())
            .get(spender)
            .unwrap_or(&0)
    }
}
