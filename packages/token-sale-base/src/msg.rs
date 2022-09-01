use create_type_spec_derive::CreateTypeSpec;
use pbc_contract_common::address::Address;
use read_write_rpc_derive::ReadWriteRPC;

use mpc20_base::{msg::InitialBalance, state::TokenInfo};
use utils::events::NamedRPCEvent;

#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Debug)]
pub struct TokenSaleInitMsg {
    pub withdraw_account: Address,
    pub deposit_mpc20_address: Address,
    pub receive_mpc20_address: Address,
}
