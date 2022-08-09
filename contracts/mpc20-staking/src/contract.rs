use crate::state::ContractState;

use pbc_contract_common::{context::ContractContext, events::EventGroup};

use mpc20_staking_base::{
    actions::{execute_claim, execute_compound, execute_init, execute_stake, execute_unstake},
    msg::{ClaimMsg, CompoundMsg, Mpc20StakingInitMsg, StakeMsg, UnstakeMsg},
};

use mpc20_base::{
    actions::{
        execute_approve as mpc20_execute_approve, execute_burn as mpc20_execute_burn,
        execute_burn_from as mpc20_execute_burn_from,
        execute_decrease_allowance as mpc20_execute_decrease_allowance,
        execute_increase_allowance as mpc20_execute_increase_allowance,
        execute_mint as mpc20_execute_mint, execute_transfer as mpc20_execute_transfer,
        execute_transfer_from as mpc20_execute_transfer_from,
    },
    msg::{
        ApproveMsg as Mpc20ApproveMsg, BurnFromMsg as Mpc20BurnFromMsg, BurnMsg as Mpc20BurnMsg,
        DecreaseAllowanceMsg as Mpc20DecreaseAllowanceMsg,
        IncreaseAllowanceMsg as Mpc20IncreaseAllowanceMsg, MintMsg as Mpc20MintMsg,
        TransferFromMsg as Mpc20TransferFromMsg, TransferMsg as Mpc20TransferMsg,
    },
};

#[init]
pub fn initialize(
    ctx: ContractContext,
    msg: Mpc20StakingInitMsg,
) -> (ContractState, Vec<EventGroup>) {
    let (mpc20_staking, events) = execute_init(&ctx, &msg);
    let state = ContractState { mpc20_staking };

    (state, events)
}

#[action]
pub fn stake(
    ctx: ContractContext,
    state: ContractState,
    msg: StakeMsg,
) -> (ContractState, Vec<EventGroup>) {
    let mut state = state;
    let events = execute_stake(&ctx, &mut state.mpc20_staking, &msg);

    (state, events)
}

#[action]
pub fn unstake(
    ctx: ContractContext,
    state: ContractState,
    msg: UnstakeMsg,
) -> (ContractState, Vec<EventGroup>) {
    let mut state = state;
    let events = execute_unstake(&ctx, &mut state.mpc20_staking, &msg);

    (state, events)
}

#[action]
pub fn claim(
    ctx: ContractContext,
    state: ContractState,
    msg: ClaimMsg,
) -> (ContractState, Vec<EventGroup>) {
    let mut state = state;
    let events = execute_claim(&ctx, &mut state.mpc20_staking, &msg);

    (state, events)
}

#[action]
pub fn compound(
    ctx: ContractContext,
    state: ContractState,
    msg: CompoundMsg,
) -> (ContractState, Vec<EventGroup>) {
    let mut state = state;
    let events = execute_compound(&ctx, &mut state.mpc20_staking, &msg);

    (state, events)
}

// ----- MPC20 Base Methods -----
#[action]
pub fn mint(
    ctx: ContractContext,
    state: ContractState,
    msg: Mpc20MintMsg,
) -> (ContractState, Vec<EventGroup>) {
    let mut state = state;
    let events = mpc20_execute_mint(&ctx, &mut state.mpc20_staking.mpc20, &msg);

    (state, events)
}

#[action]
pub fn transfer(
    ctx: ContractContext,
    state: ContractState,
    msg: Mpc20TransferMsg,
) -> (ContractState, Vec<EventGroup>) {
    let mut state = state;
    let events = mpc20_execute_transfer(&ctx, &mut state.mpc20_staking.mpc20, &msg);

    (state, events)
}

#[action]
pub fn transfer_from(
    ctx: ContractContext,
    state: ContractState,
    msg: Mpc20TransferFromMsg,
) -> (ContractState, Vec<EventGroup>) {
    let mut state = state;
    let events = mpc20_execute_transfer_from(&ctx, &mut state.mpc20_staking.mpc20, &msg);

    (state, events)
}

#[action]
pub fn burn(
    ctx: ContractContext,
    state: ContractState,
    msg: Mpc20BurnMsg,
) -> (ContractState, Vec<EventGroup>) {
    let mut state = state;
    let events = mpc20_execute_burn(&ctx, &mut state.mpc20_staking.mpc20, &msg);

    (state, events)
}

#[action]
pub fn burn_from(
    ctx: ContractContext,
    state: ContractState,
    msg: Mpc20BurnFromMsg,
) -> (ContractState, Vec<EventGroup>) {
    let mut state = state;
    let events = mpc20_execute_burn_from(&ctx, &mut state.mpc20_staking.mpc20, &msg);

    (state, events)
}

#[action]
pub fn approve(
    ctx: ContractContext,
    state: ContractState,
    msg: Mpc20ApproveMsg,
) -> (ContractState, Vec<EventGroup>) {
    let mut state = state;
    let events = mpc20_execute_approve(&ctx, &mut state.mpc20_staking.mpc20, &msg);

    (state, events)
}

#[action]
pub fn increase_allowance(
    ctx: ContractContext,
    state: ContractState,
    msg: Mpc20IncreaseAllowanceMsg,
) -> (ContractState, Vec<EventGroup>) {
    let mut state = state;
    let events = mpc20_execute_increase_allowance(&ctx, &mut state.mpc20_staking.mpc20, &msg);

    (state, events)
}

#[action]
pub fn decrease_allowance(
    ctx: ContractContext,
    state: ContractState,
    msg: Mpc20DecreaseAllowanceMsg,
) -> (ContractState, Vec<EventGroup>) {
    let mut state = state;
    let events = mpc20_execute_decrease_allowance(&ctx, &mut state.mpc20_staking.mpc20, &msg);

    (state, events)
}
