use pbc_contract_common::{context::ContractContext, events::EventGroup};

use crate::{
    actions::{execute_claim, execute_compound, execute_init, execute_stake, execute_unstake},
    msg::{ClaimMsg, CompoundMsg, InitMsg, StakeMsg, UnstakeMsg},
    state::MPC20StakingContractState,
};

use mpc20::{
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
    msg: InitMsg,
) -> (MPC20StakingContractState, Vec<EventGroup>) {
    execute_init(ctx, msg)
}

#[action]
pub fn stake(
    ctx: ContractContext,
    state: MPC20StakingContractState,
    msg: StakeMsg,
) -> (MPC20StakingContractState, Vec<EventGroup>) {
    execute_stake(ctx, state, msg)
}

#[action]
pub fn unstake(
    ctx: ContractContext,
    state: MPC20StakingContractState,
    msg: UnstakeMsg,
) -> (MPC20StakingContractState, Vec<EventGroup>) {
    execute_unstake(ctx, state, msg)
}

#[action]
pub fn claim(
    ctx: ContractContext,
    state: MPC20StakingContractState,
    msg: ClaimMsg,
) -> (MPC20StakingContractState, Vec<EventGroup>) {
    execute_claim(ctx, state, msg)
}

#[action]
pub fn compound(
    ctx: ContractContext,
    state: MPC20StakingContractState,
    msg: CompoundMsg,
) -> (MPC20StakingContractState, Vec<EventGroup>) {
    execute_compound(ctx, state, msg)
}

// ----- MPC20 Base Methods -----
#[action]
pub fn mint(
    ctx: ContractContext,
    state: MPC20StakingContractState,
    msg: Mpc20MintMsg,
) -> (MPC20StakingContractState, Vec<EventGroup>) {
    let mut state = state;
    let (mpc20_state, _) = mpc20_execute_mint(ctx, state.mpc20_base_state, msg);
    state.mpc20_base_state = mpc20_state;

    (state, vec![])
}

#[action]
pub fn transfer(
    ctx: ContractContext,
    state: MPC20StakingContractState,
    msg: Mpc20TransferMsg,
) -> (MPC20StakingContractState, Vec<EventGroup>) {
    let mut state = state;
    let (mpc20_state, _) = mpc20_execute_transfer(ctx, state.mpc20_base_state, msg);
    state.mpc20_base_state = mpc20_state;

    (state, vec![])
}

#[action]
pub fn transfer_from(
    ctx: ContractContext,
    state: MPC20StakingContractState,
    msg: Mpc20TransferFromMsg,
) -> (MPC20StakingContractState, Vec<EventGroup>) {
    let mut state = state;
    let (mpc20_state, _) = mpc20_execute_transfer_from(ctx, state.mpc20_base_state, msg);
    state.mpc20_base_state = mpc20_state;

    (state, vec![])
}

#[action]
pub fn burn(
    ctx: ContractContext,
    state: MPC20StakingContractState,
    msg: Mpc20BurnMsg,
) -> (MPC20StakingContractState, Vec<EventGroup>) {
    let mut state = state;
    let (mpc20_state, _) = mpc20_execute_burn(ctx, state.mpc20_base_state, msg);
    state.mpc20_base_state = mpc20_state;

    (state, vec![])
}

#[action]
pub fn burn_from(
    ctx: ContractContext,
    state: MPC20StakingContractState,
    msg: Mpc20BurnFromMsg,
) -> (MPC20StakingContractState, Vec<EventGroup>) {
    let mut state = state;
    let (mpc20_state, _) = mpc20_execute_burn_from(ctx, state.mpc20_base_state, msg);
    state.mpc20_base_state = mpc20_state;

    (state, vec![])
}

#[action]
pub fn approve(
    ctx: ContractContext,
    state: MPC20StakingContractState,
    msg: Mpc20ApproveMsg,
) -> (MPC20StakingContractState, Vec<EventGroup>) {
    let mut state = state;
    let (mpc20_state, _) = mpc20_execute_approve(ctx, state.mpc20_base_state, msg);
    state.mpc20_base_state = mpc20_state;

    (state, vec![])
}

#[action]
pub fn increase_allowance(
    ctx: ContractContext,
    state: MPC20StakingContractState,
    msg: Mpc20IncreaseAllowanceMsg,
) -> (MPC20StakingContractState, Vec<EventGroup>) {
    let mut state = state;
    let (mpc20_state, _) = mpc20_execute_increase_allowance(ctx, state.mpc20_base_state, msg);
    state.mpc20_base_state = mpc20_state;

    (state, vec![])
}

#[action]
pub fn decrease_allowance(
    ctx: ContractContext,
    state: MPC20StakingContractState,
    msg: Mpc20DecreaseAllowanceMsg,
) -> (MPC20StakingContractState, Vec<EventGroup>) {
    let mut state = state;
    let (mpc20_state, _) = mpc20_execute_decrease_allowance(ctx, state.mpc20_base_state, msg);
    state.mpc20_base_state = mpc20_state;

    (state, vec![])
}
