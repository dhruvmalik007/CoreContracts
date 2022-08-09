use crate::state::ContractState;

use pbc_contract_common::{context::ContractContext, events::EventGroup};

use mpc1155_base::{
    actions::{
        execute_approve_for_all, execute_batch_burn, execute_batch_mint,
        execute_batch_transfer_from, execute_burn, execute_init, execute_mint,
        execute_revoke_for_all, execute_set_uri, execute_transfer_from,
    },
    msg::{
        ApproveForAllMsg, BatchBurnMsg, BatchMintMsg, BatchTransferFromMsg, BurnMsg, InitMsg,
        MintMsg, RevokeForAllMsg, SetUriMsg, TransferFromMsg,
    },
};

#[init]
pub fn initialize(ctx: ContractContext, msg: InitMsg) -> (ContractState, Vec<EventGroup>) {
    let (mpc1155, events) = execute_init(&ctx, &msg);
    let state = ContractState { mpc1155 };

    (state, events)
}

#[action]
pub fn set_uri(
    ctx: ContractContext,
    state: ContractState,
    msg: SetUriMsg,
) -> (ContractState, Vec<EventGroup>) {
    let mut state = state;
    let events = execute_set_uri(&ctx, &mut state.mpc1155, &msg);

    (state, events)
}

#[action]
pub fn mint(
    ctx: ContractContext,
    state: ContractState,
    msg: MintMsg,
) -> (ContractState, Vec<EventGroup>) {
    let mut state = state;
    let events = execute_mint(&ctx, &mut state.mpc1155, &msg);

    (state, events)
}

#[action]
pub fn batch_mint(
    ctx: ContractContext,
    state: ContractState,
    msg: BatchMintMsg,
) -> (ContractState, Vec<EventGroup>) {
    let mut state = state;
    let events = execute_batch_mint(&ctx, &mut state.mpc1155, &msg);

    (state, events)
}

#[action]
pub fn transfer_from(
    ctx: ContractContext,
    state: ContractState,
    msg: TransferFromMsg,
) -> (ContractState, Vec<EventGroup>) {
    let mut state = state;
    let events = execute_transfer_from(&ctx, &mut state.mpc1155, &msg);

    (state, events)
}

#[action]
pub fn batch_transfer_from(
    ctx: ContractContext,
    state: ContractState,
    msg: BatchTransferFromMsg,
) -> (ContractState, Vec<EventGroup>) {
    let mut state = state;
    let events = execute_batch_transfer_from(&ctx, &mut state.mpc1155, &msg);

    (state, events)
}

#[action]
pub fn burn(
    ctx: ContractContext,
    state: ContractState,
    msg: BurnMsg,
) -> (ContractState, Vec<EventGroup>) {
    let mut state = state;
    let events = execute_burn(&ctx, &mut state.mpc1155, &msg);

    (state, events)
}

#[action]
pub fn batch_burn(
    ctx: ContractContext,
    state: ContractState,
    msg: BatchBurnMsg,
) -> (ContractState, Vec<EventGroup>) {
    let mut state = state;
    let events = execute_batch_burn(&ctx, &mut state.mpc1155, &msg);

    (state, events)
}

#[action]
pub fn approve_for_all(
    ctx: ContractContext,
    state: ContractState,
    msg: ApproveForAllMsg,
) -> (ContractState, Vec<EventGroup>) {
    let mut state = state;
    let events = execute_approve_for_all(&ctx, &mut state.mpc1155, &msg);

    (state, events)
}

#[action]
pub fn revoke_for_all(
    ctx: ContractContext,
    state: ContractState,
    msg: RevokeForAllMsg,
) -> (ContractState, Vec<EventGroup>) {
    let mut state = state;
    let events = execute_revoke_for_all(&ctx, &mut state.mpc1155, &msg);

    (state, events)
}
