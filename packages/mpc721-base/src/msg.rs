use create_type_spec_derive::CreateTypeSpec;
use pbc_contract_common::address::Address;
use read_write_rpc_derive::ReadWriteRPC;

use utils::events::NamedRPCEvent;

#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct InitMsg {
    pub owner: Option<Address>,
    pub name: String,
    pub symbol: String,
    pub base_uri: Option<String>,
    pub minter: Address,
}

#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct SetBaseUriMsg {
    pub new_base_uri: String,
}

impl NamedRPCEvent for SetBaseUriMsg {
    fn event_name(&self) -> String {
        "set_base_uri".to_string()
    }
}

#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct MintMsg {
    pub token_id: u128,
    pub to: Address,
    pub token_uri: Option<String>,
}

impl NamedRPCEvent for MintMsg {
    fn event_name(&self) -> String {
        "mint".to_string()
    }
}

#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct TransferMsg {
    pub to: Address,
    pub token_id: u128,
}

impl NamedRPCEvent for TransferMsg {
    fn event_name(&self) -> String {
        "transfer".to_string()
    }
}

#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct TransferFromMsg {
    pub from: Address,
    pub to: Address,
    pub token_id: u128,
}

impl NamedRPCEvent for TransferFromMsg {
    fn event_name(&self) -> String {
        "transfer_from".to_string()
    }
}

#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct ApproveMsg {
    pub spender: Address,
    pub token_id: u128,
}

impl NamedRPCEvent for ApproveMsg {
    fn event_name(&self) -> String {
        "approve".to_string()
    }
}

#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct ApproveForAllMsg {
    pub operator: Address,
}

impl NamedRPCEvent for ApproveForAllMsg {
    fn event_name(&self) -> String {
        "approve_for_all".to_string()
    }
}

#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct RevokeMsg {
    pub spender: Address,
    pub token_id: u128,
}

impl NamedRPCEvent for RevokeMsg {
    fn event_name(&self) -> String {
        "revoke".to_string()
    }
}

#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct RevokeForAllMsg {
    pub operator: Address,
}

impl NamedRPCEvent for RevokeForAllMsg {
    fn event_name(&self) -> String {
        "revoke_for_all".to_string()
    }
}

#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct BurnMsg {
    pub token_id: u128,
}

impl NamedRPCEvent for BurnMsg {
    fn event_name(&self) -> String {
        "burn".to_string()
    }
}
