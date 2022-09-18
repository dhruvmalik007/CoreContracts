use create_type_spec_derive::CreateTypeSpec;
use pbc_contract_common::address::Address;
use read_write_rpc_derive::ReadWriteRPC;

use utils::events::NamedRPCEvent;

/// ## Description
/// This structure describes fields for mpc721 initialize msg
#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct InitMsg {
    /// optional owner address
    pub owner: Option<Address>,
    /// token name
    pub name: String,
    /// token symbol
    pub symbol: String,
    /// optional base uri
    pub base_uri: Option<String>,
    /// token minter address
    pub minter: Address,
}

/// ## Description
/// This structure describes fields for mpc721 set base uri msg
#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct SetBaseUriMsg {
    /// new base uri
    pub new_base_uri: String,
}

impl NamedRPCEvent for SetBaseUriMsg {
    fn event_name(&self) -> String {
        "set_base_uri".to_string()
    }
}

/// ## Description
/// This structure describes fields for mpc721 mint msg
#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct MintMsg {
    /// newly minted token id
    pub token_id: u128,
    /// receiver address
    pub to: Address,
    /// optional token uri
    pub token_uri: Option<String>,
}

impl NamedRPCEvent for MintMsg {
    fn event_name(&self) -> String {
        "mint".to_string()
    }
}

/// ## Description
/// This structure describes fields for mpc721 transfer msg
#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct TransferMsg {
    /// receiver address
    pub to: Address,
    /// token id
    pub token_id: u128,
}

impl NamedRPCEvent for TransferMsg {
    fn event_name(&self) -> String {
        "transfer".to_string()
    }
}

/// ## Description
/// This structure describes fields for mpc721 transfer from msg
#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct TransferFromMsg {
    /// owner address
    pub from: Address,
    /// receiver address
    pub to: Address,
    /// token id
    pub token_id: u128,
}

impl NamedRPCEvent for TransferFromMsg {
    fn event_name(&self) -> String {
        "transfer_from".to_string()
    }
}

/// ## Description
/// This structure describes fields for mpc721 approve msg
#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct ApproveMsg {
    /// operator address to approve
    pub spender: Address,
    /// token id
    pub token_id: u128,
}

impl NamedRPCEvent for ApproveMsg {
    fn event_name(&self) -> String {
        "approve".to_string()
    }
}

/// ## Description
/// This structure describes fields for mpc721 approve for all msg
#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct ApproveForAllMsg {
    /// operator address to approve
    pub operator: Address,
}

impl NamedRPCEvent for ApproveForAllMsg {
    fn event_name(&self) -> String {
        "approve_for_all".to_string()
    }
}

/// ## Description
/// This structure describes fields for mpc721 revoke msg
#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct RevokeMsg {
    /// operator address to revoke
    pub spender: Address,
    /// token id
    pub token_id: u128,
}

impl NamedRPCEvent for RevokeMsg {
    fn event_name(&self) -> String {
        "revoke".to_string()
    }
}

/// ## Description
/// This structure describes fields for mpc721 revoke for all msg
#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct RevokeForAllMsg {
    /// operator address to revoke
    pub operator: Address,
}

impl NamedRPCEvent for RevokeForAllMsg {
    fn event_name(&self) -> String {
        "revoke_for_all".to_string()
    }
}

/// ## Description
/// This structure describes fields for mpc721 burn msg
#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct BurnMsg {
    /// token id to burn
    pub token_id: u128,
}

impl NamedRPCEvent for BurnMsg {
    fn event_name(&self) -> String {
        "burn".to_string()
    }
}
