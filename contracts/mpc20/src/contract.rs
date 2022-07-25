use pbc_contract_common::{context::ContractContext, events::EventGroup};

use crate::{
    actions::{
        execute_approve, execute_burn, execute_burn_from, execute_decrease_allowance,
        execute_increase_allowance, execute_init, execute_mint, execute_transfer,
        execute_transfer_from,
    },
    msg::{
        ApproveMsg, BurnFromMsg, BurnMsg, DecreaseAllowanceMsg, IncreaseAllowanceMsg, InitMsg,
        MintMsg, TransferFromMsg, TransferMsg,
    },
    state::MPC20ContractState,
};

#[init]
pub fn initialize(ctx: ContractContext, msg: InitMsg) -> (MPC20ContractState, Vec<EventGroup>) {
    execute_init(ctx, msg)
}

#[action]
pub fn mint(
    ctx: ContractContext,
    state: MPC20ContractState,
    msg: MintMsg,
) -> (MPC20ContractState, Vec<EventGroup>) {
    execute_mint(ctx, state, msg)
}

#[action]
pub fn transfer(
    ctx: ContractContext,
    state: MPC20ContractState,
    msg: TransferMsg,
) -> (MPC20ContractState, Vec<EventGroup>) {
    execute_transfer(ctx, state, msg)
}

#[action]
pub fn transfer_from(
    ctx: ContractContext,
    state: MPC20ContractState,
    msg: TransferFromMsg,
) -> (MPC20ContractState, Vec<EventGroup>) {
    execute_transfer_from(ctx, state, msg)
}

#[action]
pub fn burn(
    ctx: ContractContext,
    state: MPC20ContractState,
    msg: BurnMsg,
) -> (MPC20ContractState, Vec<EventGroup>) {
    execute_burn(ctx, state, msg)
}

#[action]
pub fn burn_from(
    ctx: ContractContext,
    state: MPC20ContractState,
    msg: BurnFromMsg,
) -> (MPC20ContractState, Vec<EventGroup>) {
    execute_burn_from(ctx, state, msg)
}

#[action]
pub fn approve(
    ctx: ContractContext,
    state: MPC20ContractState,
    msg: ApproveMsg,
) -> (MPC20ContractState, Vec<EventGroup>) {
    execute_approve(ctx, state, msg)
}

#[action]
pub fn increase_allowance(
    ctx: ContractContext,
    state: MPC20ContractState,
    msg: IncreaseAllowanceMsg,
) -> (MPC20ContractState, Vec<EventGroup>) {
    execute_increase_allowance(ctx, state, msg)
}

#[action]
pub fn decrease_allowance(
    ctx: ContractContext,
    state: MPC20ContractState,
    msg: DecreaseAllowanceMsg,
) -> (MPC20ContractState, Vec<EventGroup>) {
    execute_decrease_allowance(ctx, state, msg)
}
