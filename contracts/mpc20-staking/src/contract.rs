use crate::state::ContractState;

use contract_version_base::state::ContractVersionBase;
use pbc_contract_common::{address::Address, context::ContractContext, events::EventGroup};

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

const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[init]
pub fn initialize(
    ctx: ContractContext,
    msg: Mpc20StakingInitMsg,
) -> (ContractState, Vec<EventGroup>) {
    let (mpc20_staking, events) = execute_init(&ctx, &msg);
    let state = ContractState {
        mpc20_staking,
        version: ContractVersionBase::new(CONTRACT_NAME, CONTRACT_VERSION),
    };

    (state, events)
}

#[action(shortname = 0x17)]
pub fn stake(
    ctx: ContractContext,
    state: ContractState,
    amount: u128,
) -> (ContractState, Vec<EventGroup>) {
    let mut state = state;
    let events = execute_stake(&ctx, &mut state.mpc20_staking, &StakeMsg { amount });

    (state, events)
}

#[action(shortname = 0x19)]
pub fn unstake(
    ctx: ContractContext,
    state: ContractState,
    amount: u128,
) -> (ContractState, Vec<EventGroup>) {
    let mut state = state;
    let events = execute_unstake(&ctx, &mut state.mpc20_staking, &UnstakeMsg { amount });

    (state, events)
}

#[action(shortname = 0x21)]
pub fn claim(
    ctx: ContractContext,
    state: ContractState,
    amount: Option<u128>,
) -> (ContractState, Vec<EventGroup>) {
    let mut state = state;
    let events = execute_claim(&ctx, &mut state.mpc20_staking, &ClaimMsg { amount });

    (state, events)
}

#[action(shortname = 0x23)]
pub fn compound(
    ctx: ContractContext,
    state: ContractState,
    amount: Option<u128>,
) -> (ContractState, Vec<EventGroup>) {
    let mut state = state;
    let events = execute_compound(&ctx, &mut state.mpc20_staking, &CompoundMsg { amount });

    (state, events)
}

// ----- MPC20 Base Methods -----
#[action(shortname = 0x01)]
pub fn transfer(
    ctx: ContractContext,
    state: ContractState,
    to: Address,
    amount: u128,
) -> (ContractState, Vec<EventGroup>) {
    let mut state = state;
    let events = mpc20_execute_transfer(
        &ctx,
        &mut state.mpc20_staking.mpc20,
        &Mpc20TransferMsg { to, amount },
    );

    (state, events)
}

#[action(shortname = 0x03)]
pub fn transfer_from(
    ctx: ContractContext,
    state: ContractState,
    from: Address,
    to: Address,
    amount: u128,
) -> (ContractState, Vec<EventGroup>) {
    let mut state = state;
    let events = mpc20_execute_transfer_from(
        &ctx,
        &mut state.mpc20_staking.mpc20,
        &Mpc20TransferFromMsg { from, to, amount },
    );

    (state, events)
}

#[action(shortname = 0x05)]
pub fn approve(
    ctx: ContractContext,
    state: ContractState,
    spender: Address,
    amount: u128,
) -> (ContractState, Vec<EventGroup>) {
    let mut state = state;
    let events = mpc20_execute_approve(
        &ctx,
        &mut state.mpc20_staking.mpc20,
        &Mpc20ApproveMsg { spender, amount },
    );

    (state, events)
}

#[action(shortname = 0x07)]
pub fn mint(
    ctx: ContractContext,
    state: ContractState,
    recipient: Address,
    amount: u128,
) -> (ContractState, Vec<EventGroup>) {
    let mut state = state;
    let events = mpc20_execute_mint(
        &ctx,
        &mut state.mpc20_staking.mpc20,
        &Mpc20MintMsg { recipient, amount },
    );

    (state, events)
}

#[action(shortname = 0x09)]
pub fn burn(
    ctx: ContractContext,
    state: ContractState,
    amount: u128,
) -> (ContractState, Vec<EventGroup>) {
    let mut state = state;
    let events = mpc20_execute_burn(
        &ctx,
        &mut state.mpc20_staking.mpc20,
        &Mpc20BurnMsg { amount },
    );

    (state, events)
}

#[action(shortname = 0x11)]
pub fn burn_from(
    ctx: ContractContext,
    state: ContractState,
    owner: Address,
    amount: u128,
) -> (ContractState, Vec<EventGroup>) {
    let mut state = state;
    let events = mpc20_execute_burn_from(
        &ctx,
        &mut state.mpc20_staking.mpc20,
        &Mpc20BurnFromMsg { owner, amount },
    );

    (state, events)
}

#[action(shortname = 0x13)]
pub fn increase_allowance(
    ctx: ContractContext,
    state: ContractState,
    spender: Address,
    amount: u128,
) -> (ContractState, Vec<EventGroup>) {
    let mut state = state;
    let events = mpc20_execute_increase_allowance(
        &ctx,
        &mut state.mpc20_staking.mpc20,
        &Mpc20IncreaseAllowanceMsg { spender, amount },
    );

    (state, events)
}

#[action(shortname = 0x15)]
pub fn decrease_allowance(
    ctx: ContractContext,
    state: ContractState,
    spender: Address,
    amount: u128,
) -> (ContractState, Vec<EventGroup>) {
    let mut state = state;
    let events = mpc20_execute_decrease_allowance(
        &ctx,
        &mut state.mpc20_staking.mpc20,
        &Mpc20DecreaseAllowanceMsg { spender, amount },
    );

    (state, events)
}
