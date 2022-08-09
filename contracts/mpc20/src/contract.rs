use crate::state::ContractState;

use pbc_contract_common::{context::ContractContext, events::EventGroup};

use mpc20_base::{
    actions::{
        execute_approve, execute_burn, execute_burn_from, execute_decrease_allowance,
        execute_increase_allowance, execute_init, execute_mint, execute_transfer,
        execute_transfer_from,
    },
    msg::{
        ApproveMsg, BurnFromMsg, BurnMsg, DecreaseAllowanceMsg, IncreaseAllowanceMsg, MintMsg,
        Mpc20InitMsg, TransferFromMsg, TransferMsg,
    },
};

#[init]
pub fn initialize(ctx: ContractContext, msg: Mpc20InitMsg) -> (ContractState, Vec<EventGroup>) {
    let (mpc20, events) = execute_init(&ctx, &msg);
    let state = ContractState { mpc20 };

    (state, events)
}

#[action]
pub fn mint(
    ctx: ContractContext,
    state: ContractState,
    msg: MintMsg,
) -> (ContractState, Vec<EventGroup>) {
    let mut state = state;
    let events = execute_mint(&ctx, &mut state.mpc20, &msg);

    (state, events)
}

#[action]
pub fn transfer(
    ctx: ContractContext,
    state: ContractState,
    msg: TransferMsg,
) -> (ContractState, Vec<EventGroup>) {
    let mut state = state;
    let events = execute_transfer(&ctx, &mut state.mpc20, &msg);

    (state, events)
}

#[action]
pub fn transfer_from(
    ctx: ContractContext,
    state: ContractState,
    msg: TransferFromMsg,
) -> (ContractState, Vec<EventGroup>) {
    let mut state = state;
    let events = execute_transfer_from(&ctx, &mut state.mpc20, &msg);

    (state, events)
}

#[action]
pub fn burn(
    ctx: ContractContext,
    state: ContractState,
    msg: BurnMsg,
) -> (ContractState, Vec<EventGroup>) {
    let mut state = state;
    let events = execute_burn(&ctx, &mut state.mpc20, &msg);

    (state, events)
}

#[action]
pub fn burn_from(
    ctx: ContractContext,
    state: ContractState,
    msg: BurnFromMsg,
) -> (ContractState, Vec<EventGroup>) {
    let mut state = state;
    let events = execute_burn_from(&ctx, &mut state.mpc20, &msg);

    (state, events)
}

#[action]
pub fn approve(
    ctx: ContractContext,
    state: ContractState,
    msg: ApproveMsg,
) -> (ContractState, Vec<EventGroup>) {
    let mut state = state;
    let events = execute_approve(&ctx, &mut state.mpc20, &msg);

    (state, events)
}

#[action]
pub fn increase_allowance(
    ctx: ContractContext,
    state: ContractState,
    msg: IncreaseAllowanceMsg,
) -> (ContractState, Vec<EventGroup>) {
    let mut state = state;
    let events = execute_increase_allowance(&ctx, &mut state.mpc20, &msg);

    (state, events)
}

#[action]
pub fn decrease_allowance(
    ctx: ContractContext,
    state: ContractState,
    msg: DecreaseAllowanceMsg,
) -> (ContractState, Vec<EventGroup>) {
    let mut state = state;
    let events = execute_decrease_allowance(&ctx, &mut state.mpc20, &msg);

    (state, events)
}
