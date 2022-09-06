use create_type_spec_derive::CreateTypeSpec;
use pbc_contract_common::address::Address;
use read_write_rpc_derive::ReadWriteRPC;

use utils::events::NamedRPCEvent;

#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct InitMsg {
    pub owner: Option<Address>,
    pub uri: String,
    pub minter: Address,
}

#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct SetUriMsg {
    pub new_uri: String,
}

impl NamedRPCEvent for SetUriMsg {
    fn event_name(&self) -> String {
        "set_uri".to_string()
    }
}

#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct TokenMintInfoMsg {
    pub token_id: u128,
    pub amount: u128,
    pub token_uri: Option<String>,
}

#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct MintMsg {
    pub to: Address,
    pub token_info: TokenMintInfoMsg,
}

impl NamedRPCEvent for MintMsg {
    fn event_name(&self) -> String {
        "mint".to_string()
    }
}

#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct BatchMintMsg {
    pub to: Address,
    pub token_infos: Vec<TokenMintInfoMsg>,
}

impl NamedRPCEvent for BatchMintMsg {
    fn event_name(&self) -> String {
        "batch_mint".to_string()
    }
}

#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct TokenTransferInfoMsg {
    pub token_id: u128,
    pub amount: u128,
}

#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct TransferFromMsg {
    pub from: Address,
    pub to: Address,
    pub token_info: TokenTransferInfoMsg,
}

#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct BatchTransferFromMsg {
    pub from: Address,
    pub to: Address,
    pub token_infos: Vec<TokenTransferInfoMsg>,
}

#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct BurnMsg {
    pub from: Address,
    pub token_info: TokenTransferInfoMsg,
}

#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct BatchBurnMsg {
    pub from: Address,
    pub token_infos: Vec<TokenTransferInfoMsg>,
}

#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct ApproveForAllMsg {
    pub operator: Address,
}

#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct RevokeForAllMsg {
    pub operator: Address,
}
