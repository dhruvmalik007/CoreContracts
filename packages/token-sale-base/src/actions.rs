use std::collections::BTreeMap;

use pbc_contract_common::{context::ContractContext, events::EventGroup};
use rust_decimal::prelude::*;
use utils::events::into_rpc_call;

use crate::{
    msg::{TokenSaleInitMsg},
    state::TokenSaleContractState,
    ContractError,
    actions
};

use ownable_base::{
    state::OwnableBaseState,
};

use utils::merkle::{validate_merkle_root, verify_merkle_proof};

pub fn execute_init(
    ctx: &ContractContext,
    msg: &TokenSaleInitMsg,
) -> (TokenSaleContractState, Vec<EventGroup>) {

    let ownable = OwnableBaseState::new(ctx);

    let state = TokenSaleContractState{
        deposit_mpc20_address: msg.deposit_mpc20_address,
        receive_mpc20_address: msg.receive_mpc20_address,
        withdraw_account: msg.withdraw_account,
        whitelisting_tiers: BTreeMap::new(),
        deposits: BTreeMap::new(),
        payouts: BTreeMap::new(),
        ownable,
    };
    (state, vec![])
}
