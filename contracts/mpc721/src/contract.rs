use crate::state::ContractState;

use pbc_contract_common::{context::ContractContext, events::EventGroup};

use mpc721_base::{
    actions::{
        execute_approve, execute_approve_for_all, execute_burn, execute_init, execute_mint,
        execute_revoke, execute_revoke_for_all, execute_set_base_uri, execute_transfer,
        execute_transfer_from,
    },
    msg::{
        ApproveForAllMsg, ApproveMsg, BurnMsg, InitMsg, MintMsg, RevokeForAllMsg, RevokeMsg,
        SetBaseUriMsg, TransferFromMsg, TransferMsg,
    },
};

#[init]
pub fn initialize(ctx: ContractContext, msg: InitMsg) -> (ContractState, Vec<EventGroup>) {
    let (mpc721, events) = execute_init(&ctx, &msg);
    let state = ContractState { mpc721 };

    (state, events)
}

#[action]
pub fn set_base_uri(
    ctx: ContractContext,
    state: ContractState,
    msg: SetBaseUriMsg,
) -> (ContractState, Vec<EventGroup>) {
    let mut state = state;
    let events = execute_set_base_uri(&ctx, &mut state.mpc721, &msg);

    (state, events)
}

#[action]
pub fn mint(
    ctx: ContractContext,
    state: ContractState,
    msg: MintMsg,
) -> (ContractState, Vec<EventGroup>) {
    let mut state = state;
    let events = execute_mint(&ctx, &mut state.mpc721, &msg);

    (state, events)
}

#[action]
pub fn transfer(
    ctx: ContractContext,
    state: ContractState,
    msg: TransferMsg,
) -> (ContractState, Vec<EventGroup>) {
    let mut state = state;
    let events = execute_transfer(&ctx, &mut state.mpc721, &msg);

    (state, events)
}

#[action]
pub fn transfer_from(
    ctx: ContractContext,
    state: ContractState,
    msg: TransferFromMsg,
) -> (ContractState, Vec<EventGroup>) {
    let mut state = state;
    let events = execute_transfer_from(&ctx, &mut state.mpc721, &msg);

    (state, events)
}

#[action]
pub fn approve(
    ctx: ContractContext,
    state: ContractState,
    msg: ApproveMsg,
) -> (ContractState, Vec<EventGroup>) {
    let mut state = state;
    let events = execute_approve(&ctx, &mut state.mpc721, &msg);

    (state, events)
}

#[action]
pub fn approve_for_all(
    ctx: ContractContext,
    state: ContractState,
    msg: ApproveForAllMsg,
) -> (ContractState, Vec<EventGroup>) {
    let mut state = state;
    let events = execute_approve_for_all(&ctx, &mut state.mpc721, &msg);

    (state, events)
}

#[action]
pub fn revoke(
    ctx: ContractContext,
    state: ContractState,
    msg: RevokeMsg,
) -> (ContractState, Vec<EventGroup>) {
    let mut state = state;
    let events = execute_revoke(&ctx, &mut state.mpc721, &msg);

    (state, events)
}

#[action]
pub fn revoke_for_all(
    ctx: ContractContext,
    state: ContractState,
    msg: RevokeForAllMsg,
) -> (ContractState, Vec<EventGroup>) {
    let mut state = state;
    let events = execute_revoke_for_all(&ctx, &mut state.mpc721, &msg);

    (state, events)
}

#[action]
pub fn burn(
    ctx: ContractContext,
    state: ContractState,
    msg: BurnMsg,
) -> (ContractState, Vec<EventGroup>) {
    let mut state = state;
    let events = execute_burn(&ctx, &mut state.mpc721, &msg);

    (state, events)
}
