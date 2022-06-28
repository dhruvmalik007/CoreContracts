use create_type_spec_derive::CreateTypeSpec;
use pbc_contract_common::address::Address;
use read_write_rpc_derive::ReadWriteRPC;

use crate::state::{Minter, TokenInfo};

use utils::events::NamedRPCEvent;

#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Debug)]
pub struct InitialBalance {
    pub address: Address,
    pub amount: u128,
}

#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Debug)]
pub struct InitMsg {
    pub info: TokenInfo,
    pub initial_balances: Vec<InitialBalance>,
    pub minter: Option<Minter>,
}

impl InitMsg {
    pub fn capacity(&self) -> Option<u128> {
        self.minter.as_ref().and_then(|m| m.capacity)
    }

    pub fn validate(&self) {
        self.validate_name();
        self.validate_symbol();
        assert!(self.info.decimals <= 18, "Decimals must not exceed 18");

        self.validate_initial_balances();
    }

    fn validate_name(&self) {
        let bytes = self.info.name.as_bytes();
        assert!(
            bytes.len() >= 3 && bytes.len() <= 50,
            "Name is not in the expected length. Must be 3-50"
        );
    }

    fn validate_symbol(&self) {
        let bytes = self.info.symbol.as_bytes();
        assert!(
            bytes.len() >= 3 && bytes.len() <= 12,
            "Ticker symbol is not in expected length. Must be 3-12"
        );

        for byte in bytes.iter() {
            if (*byte != 45) && (*byte < 65 || *byte > 90) && (*byte < 97 || *byte > 122) {
                panic!("Ticker symbol is not in expected format. Must be [a-zA-Z\\-]")
            }
        }
    }

    fn validate_initial_balances(&self) {
        let mut addrs = self
            .initial_balances
            .clone()
            .into_iter()
            .map(|b| b.address)
            .collect::<Vec<_>>();
        addrs.sort();
        addrs.dedup();

        assert!(
            addrs.len() == self.initial_balances.len(),
            "Duplicate addresses in initial balances list"
        );
    }
}

#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Debug)]
pub struct MintMsg {
    pub recipient: Address,
    pub amount: u128,
}

impl NamedRPCEvent for MintMsg {
    fn event_name(&self) -> String {
        "mint".to_string()
    }
}

#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Debug)]
pub struct TransferMsg {
    pub to: Address,
    pub amount: u128,
}

impl NamedRPCEvent for TransferMsg {
    fn event_name(&self) -> String {
        "transfer".to_string()
    }
}

#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Debug)]
pub struct TransferFromMsg {
    pub owner: Address,
    pub to: Address,
    pub amount: u128,
}

impl NamedRPCEvent for TransferFromMsg {
    fn event_name(&self) -> String {
        "transfer_from".to_string()
    }
}

#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Debug)]
pub struct BurnMsg {
    pub amount: u128,
}

impl NamedRPCEvent for BurnMsg {
    fn event_name(&self) -> String {
        "burn".to_string()
    }
}

#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Debug)]
pub struct BurnFromMsg {
    pub owner: Address,
    pub amount: u128,
}

impl NamedRPCEvent for BurnFromMsg {
    fn event_name(&self) -> String {
        "burn_from".to_string()
    }
}

#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Debug)]
pub struct ApproveMsg {
    pub spender: Address,
    pub amount: u128,
}

impl NamedRPCEvent for ApproveMsg {
    fn event_name(&self) -> String {
        "approve".to_string()
    }
}

#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Debug)]
pub struct IncreaseAllowanceMsg {
    pub spender: Address,
    pub amount: u128,
}

impl NamedRPCEvent for IncreaseAllowanceMsg {
    fn event_name(&self) -> String {
        "increase_allowance".to_string()
    }
}

#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Debug)]
pub struct DecreaseAllowanceMsg {
    pub spender: Address,
    pub amount: u128,
}

impl NamedRPCEvent for DecreaseAllowanceMsg {
    fn event_name(&self) -> String {
        "decrease_allowance".to_string()
    }
}
