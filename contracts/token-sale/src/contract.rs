use crate::state::ContractState;

use pbc_contract_common::{context::ContractContext, events::EventGroup};

use token_sale_base::{
    actions::{execute_init},
    msg::{TokenSaleInitMsg},
};

use ownable_base::{
    actions::{execute_init},
    msg::{TokenSaleInitMsg},
};

#[init]
pub fn initialize(
    ctx: ContractContext,
    msg: TokenSaleInitMsg,
) -> (ContractState, Vec<EventGroup>) {
    let (token_sale, events) = execute_init(&ctx, &msg);
    let state = ContractState { token_sale };

    (state, events)
}