use create_type_spec_derive::CreateTypeSpec;
use pbc_contract_common::address::Address;
use read_write_rpc_derive::ReadWriteRPC;

use mpc20_base::{msg::InitialBalance, state::TokenInfo};
use utils::events::NamedRPCEvent;

#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct Mpc20StakingInitMsg {
    pub deposit_token: Option<Address>,
    pub distribution_amount: u128,
    pub distribution_epoch: u64,
    pub compound_frequency: u64,
    // mpc20 base info
    pub info: TokenInfo,
    pub initial_balances: Vec<InitialBalance>,
    pub minter: Option<Address>,
}

impl Mpc20StakingInitMsg {
    pub fn validate(&self) {
        assert!(
            self.distribution_epoch > 0,
            "Distribution epoch must be higher then 0"
        );
        assert!(
            self.distribution_amount > 0,
            "Distribution amount must be higher then 0"
        )
    }
}

#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct StakeMsg {
    pub amount: u128,
}

impl NamedRPCEvent for StakeMsg {
    fn event_name(&self) -> String {
        "stake".to_string()
    }
}

#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct UnstakeMsg {
    pub amount: u128,
}

impl NamedRPCEvent for UnstakeMsg {
    fn event_name(&self) -> String {
        "unstake".to_string()
    }
}

#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct ClaimMsg {
    pub amount: Option<u128>,
}

impl NamedRPCEvent for ClaimMsg {
    fn event_name(&self) -> String {
        "claim".to_string()
    }
}

#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct CompoundMsg {
    pub amount: Option<u128>,
}

impl NamedRPCEvent for CompoundMsg {
    fn event_name(&self) -> String {
        "compound".to_string()
    }
}
