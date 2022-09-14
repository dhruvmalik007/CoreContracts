use create_type_spec_derive::CreateTypeSpec;
use pbc_contract_common::address::Address;
use read_write_rpc_derive::ReadWriteRPC;

use mpc20_base::{msg::InitialBalance, state::TokenInfo};
use utils::events::NamedRPCEvent;

/// ## Description
/// This structure describes fields for mpc20-staking initialize msg
#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct Mpc20StakingInitMsg {
    /// deposit token address, if None then deposit token will contract address
    pub deposit_token: Option<Address>,
    /// per epoch distribution amount
    pub distribution_amount: u128,
    /// UTC timestamp
    pub distribution_epoch: u64,
    /// compounding limit
    pub compound_frequency: u64,
    /// mpc20 base token info
    pub info: TokenInfo,
    /// mpc20 base initial balances
    pub initial_balances: Vec<InitialBalance>,
    /// mpc20 base optional minter address
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

/// ## Description
/// This structure describes fields for mpc20-staking stake msg
#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct StakeMsg {
    /// amount to stake
    pub amount: u128,
}

impl NamedRPCEvent for StakeMsg {
    fn event_name(&self) -> String {
        "stake".to_string()
    }
}

/// ## Description
/// This structure describes fields for mpc20-staking unstake msg
#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct UnstakeMsg {
    /// amount to unstake
    pub amount: u128,
}

impl NamedRPCEvent for UnstakeMsg {
    fn event_name(&self) -> String {
        "unstake".to_string()
    }
}

/// ## Description
/// This structure describes fields for mpc20-staking claim msg
#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct ClaimMsg {
    /// optional amount to claim, if None everything will be claimed
    pub amount: Option<u128>,
}

impl NamedRPCEvent for ClaimMsg {
    fn event_name(&self) -> String {
        "claim".to_string()
    }
}

/// ## Description
/// This structure describes fields for mpc20-staking compound msg
#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct CompoundMsg {
    /// optional amount to claim, if None everything will be compounded
    pub amount: Option<u128>,
}

impl NamedRPCEvent for CompoundMsg {
    fn event_name(&self) -> String {
        "compound".to_string()
    }
}
