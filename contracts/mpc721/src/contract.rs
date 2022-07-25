use pbc_contract_common::{context::ContractContext, events::EventGroup};

use crate::{
    actions::{
        execute_approve, execute_approve_for_all, execute_burn, execute_init, execute_mint,
        execute_revoke, execute_revoke_for_all, execute_set_base_uri, execute_transfer,
        execute_transfer_from,
    },
    msg::{
        ApproveForAllMsg, ApproveMsg, BurnMsg, InitMsg, MintMsg, RevokeForAllMsg, RevokeMsg,
        SetBaseUriMsg, TransferFromMsg, TransferMsg,
    },
    state::MPC721ContractState,
};

#[init]
pub fn initialize(ctx: ContractContext, msg: InitMsg) -> (MPC721ContractState, Vec<EventGroup>) {
    execute_init(ctx, msg)
}

#[action]
pub fn set_base_uri(
    ctx: ContractContext,
    state: MPC721ContractState,
    msg: SetBaseUriMsg,
) -> (MPC721ContractState, Vec<EventGroup>) {
    execute_set_base_uri(ctx, state, msg)
}

#[action]
pub fn mint(
    ctx: ContractContext,
    state: MPC721ContractState,
    msg: MintMsg,
) -> (MPC721ContractState, Vec<EventGroup>) {
    execute_mint(ctx, state, msg)
}

#[action]
pub fn transfer(
    ctx: ContractContext,
    state: MPC721ContractState,
    msg: TransferMsg,
) -> (MPC721ContractState, Vec<EventGroup>) {
    execute_transfer(ctx, state, msg)
}

#[action]
pub fn transfer_from(
    ctx: ContractContext,
    state: MPC721ContractState,
    msg: TransferFromMsg,
) -> (MPC721ContractState, Vec<EventGroup>) {
    execute_transfer_from(ctx, state, msg)
}

#[action]
pub fn approve(
    ctx: ContractContext,
    state: MPC721ContractState,
    msg: ApproveMsg,
) -> (MPC721ContractState, Vec<EventGroup>) {
    execute_approve(ctx, state, msg)
}

#[action]
pub fn approve_for_all(
    ctx: ContractContext,
    state: MPC721ContractState,
    msg: ApproveForAllMsg,
) -> (MPC721ContractState, Vec<EventGroup>) {
    execute_approve_for_all(ctx, state, msg)
}

#[action]
pub fn revoke(
    ctx: ContractContext,
    state: MPC721ContractState,
    msg: RevokeMsg,
) -> (MPC721ContractState, Vec<EventGroup>) {
    execute_revoke(ctx, state, msg)
}

#[action]
pub fn revoke_for_all(
    ctx: ContractContext,
    state: MPC721ContractState,
    msg: RevokeForAllMsg,
) -> (MPC721ContractState, Vec<EventGroup>) {
    execute_revoke_for_all(ctx, state, msg)
}

#[action]
pub fn burn(
    ctx: ContractContext,
    state: MPC721ContractState,
    msg: BurnMsg,
) -> (MPC721ContractState, Vec<EventGroup>) {
    execute_burn(ctx, state, msg)
}
