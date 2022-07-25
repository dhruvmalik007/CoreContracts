use pbc_contract_common::{context::ContractContext, events::EventGroup};

use crate::{
    actions::{
        execute_approve_for_all, execute_batch_burn, execute_batch_mint,
        execute_batch_transfer_from, execute_burn, execute_init, execute_mint,
        execute_revoke_for_all, execute_set_uri, execute_transfer_from,
    },
    msg::{
        ApproveForAllMsg, BatchBurnMsg, BatchMintMsg, BatchTransferFromMsg, BurnMsg, InitMsg,
        MintMsg, RevokeForAllMsg, SetUriMsg, TransferFromMsg,
    },
    state::MPC1155ContractState,
};

#[init]
pub fn initialize(ctx: ContractContext, msg: InitMsg) -> (MPC1155ContractState, Vec<EventGroup>) {
    execute_init(ctx, msg)
}

#[action]
pub fn set_uri(
    ctx: ContractContext,
    state: MPC1155ContractState,
    msg: SetUriMsg,
) -> (MPC1155ContractState, Vec<EventGroup>) {
    execute_set_uri(ctx, state, &msg)
}

#[action]
pub fn mint(
    ctx: ContractContext,
    state: MPC1155ContractState,
    msg: MintMsg,
) -> (MPC1155ContractState, Vec<EventGroup>) {
    execute_mint(ctx, state, &msg)
}

#[action]
pub fn batch_mint(
    ctx: ContractContext,
    state: MPC1155ContractState,
    msg: BatchMintMsg,
) -> (MPC1155ContractState, Vec<EventGroup>) {
    execute_batch_mint(ctx, state, &msg)
}

#[action]
pub fn transfer_from(
    ctx: ContractContext,
    state: MPC1155ContractState,
    msg: TransferFromMsg,
) -> (MPC1155ContractState, Vec<EventGroup>) {
    execute_transfer_from(ctx, state, &msg)
}

#[action]
pub fn batch_transfer_from(
    ctx: ContractContext,
    state: MPC1155ContractState,
    msg: BatchTransferFromMsg,
) -> (MPC1155ContractState, Vec<EventGroup>) {
    execute_batch_transfer_from(ctx, state, &msg)
}

#[action]
pub fn burn(
    ctx: ContractContext,
    state: MPC1155ContractState,
    msg: BurnMsg,
) -> (MPC1155ContractState, Vec<EventGroup>) {
    execute_burn(ctx, state, &msg)
}

#[action]
pub fn batch_burn(
    ctx: ContractContext,
    state: MPC1155ContractState,
    msg: BatchBurnMsg,
) -> (MPC1155ContractState, Vec<EventGroup>) {
    execute_batch_burn(ctx, state, &msg)
}

#[action]
pub fn approve_for_all(
    ctx: ContractContext,
    state: MPC1155ContractState,
    msg: ApproveForAllMsg,
) -> (MPC1155ContractState, Vec<EventGroup>) {
    execute_approve_for_all(ctx, state, &msg)
}

#[action]
pub fn revoke_for_all(
    ctx: ContractContext,
    state: MPC1155ContractState,
    msg: RevokeForAllMsg,
) -> (MPC1155ContractState, Vec<EventGroup>) {
    execute_revoke_for_all(ctx, state, &msg)
}
