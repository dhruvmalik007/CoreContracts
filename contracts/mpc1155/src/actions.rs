use std::collections::BTreeMap;

use pbc_contract_common::{context::ContractContext, events::EventGroup};

use crate::{
    msg::{
        ApproveForAllMsg, BatchBurnMsg, BatchMintMsg, BatchTransferFromMsg, BurnMsg, InitMsg,
        MintMsg, RevokeForAllMsg, SetUriMsg, TransferFromMsg,
    },
    state::{MPC1155ContractState, TokenInfo},
    ContractError,
};

pub fn execute_init(
    _ctx: ContractContext,
    msg: InitMsg,
) -> (MPC1155ContractState, Vec<EventGroup>) {
    let state = MPC1155ContractState {
        owner: msg.owner,
        uri: msg.uri,
        minter: msg.minter,
        balances: BTreeMap::new(),
        operator_approvals: BTreeMap::new(),
        tokens: BTreeMap::new(),
    };

    (state, vec![])
}

pub fn execute_set_uri(
    ctx: ContractContext,
    state: MPC1155ContractState,
    msg: &SetUriMsg,
) -> (MPC1155ContractState, Vec<EventGroup>) {
    assert!(
        state.is_owner(&ctx.sender),
        "{}",
        ContractError::Unauthorized
    );

    let mut state = state;
    state.set_uri(&msg.new_uri);

    (state, vec![])
}

pub fn execute_mint(
    ctx: ContractContext,
    state: MPC1155ContractState,
    msg: &MintMsg,
) -> (MPC1155ContractState, Vec<EventGroup>) {
    assert!(
        state.minter == ctx.sender,
        "{}",
        ContractError::Unauthorized
    );

    let mut state = state;
    state.store_token(
        msg.token_info.token_id,
        &TokenInfo {
            token_uri: msg.token_info.token_uri.clone(),
        },
    );
    state.transfer(
        None,
        Some(&msg.to),
        msg.token_info.token_id,
        msg.token_info.amount,
    );

    (state, vec![])
}

pub fn execute_batch_mint(
    ctx: ContractContext,
    state: MPC1155ContractState,
    msg: &BatchMintMsg,
) -> (MPC1155ContractState, Vec<EventGroup>) {
    assert!(
        state.minter == ctx.sender,
        "{}",
        ContractError::Unauthorized
    );

    let mut state = state;
    for token_info in msg.token_infos.iter() {
        state.store_token(
            token_info.token_id,
            &TokenInfo {
                token_uri: token_info.token_uri.clone(),
            },
        );
        state.transfer(None, Some(&msg.to), token_info.token_id, token_info.amount);
    }

    (state, vec![])
}

pub fn execute_transfer_from(
    ctx: ContractContext,
    state: MPC1155ContractState,
    msg: &TransferFromMsg,
) -> (MPC1155ContractState, Vec<EventGroup>) {
    assert!(
        state.is_token_owner_or_operator(&msg.from, &ctx.sender),
        "{}",
        ContractError::Unauthorized,
    );

    let mut state = state;
    state.transfer(
        Some(&msg.from),
        Some(&msg.to),
        msg.token_info.token_id,
        msg.token_info.amount,
    );

    (state, vec![])
}

pub fn execute_batch_transfer_from(
    ctx: ContractContext,
    state: MPC1155ContractState,
    msg: &BatchTransferFromMsg,
) -> (MPC1155ContractState, Vec<EventGroup>) {
    assert!(
        state.is_token_owner_or_operator(&msg.from, &ctx.sender),
        "{}",
        ContractError::Unauthorized,
    );

    let mut state = state;
    for token_info in msg.token_infos.iter() {
        state.transfer(
            Some(&msg.from),
            Some(&msg.to),
            token_info.token_id,
            token_info.amount,
        );
    }

    (state, vec![])
}

pub fn execute_burn(
    ctx: ContractContext,
    state: MPC1155ContractState,
    msg: &BurnMsg,
) -> (MPC1155ContractState, Vec<EventGroup>) {
    assert!(
        state.is_token_owner_or_operator(&msg.from, &ctx.sender),
        "{}",
        ContractError::Unauthorized,
    );

    let mut state = state;
    state.transfer(
        Some(&msg.from),
        None,
        msg.token_info.token_id,
        msg.token_info.amount,
    );

    (state, vec![])
}

pub fn execute_batch_burn(
    ctx: ContractContext,
    state: MPC1155ContractState,
    msg: &BatchBurnMsg,
) -> (MPC1155ContractState, Vec<EventGroup>) {
    assert!(
        state.is_token_owner_or_operator(&msg.from, &ctx.sender),
        "{}",
        ContractError::Unauthorized,
    );

    let mut state = state;
    for token_info in msg.token_infos.iter() {
        state.transfer(
            Some(&msg.from),
            None,
            token_info.token_id,
            token_info.amount,
        );
    }

    (state, vec![])
}

pub fn execute_approve_for_all(
    ctx: ContractContext,
    state: MPC1155ContractState,
    msg: &ApproveForAllMsg,
) -> (MPC1155ContractState, Vec<EventGroup>) {
    let mut state = state;
    state.add_operator(&ctx.sender, &msg.operator);

    (state, vec![])
}

pub fn execute_revoke_for_all(
    ctx: ContractContext,
    state: MPC1155ContractState,
    msg: &RevokeForAllMsg,
) -> (MPC1155ContractState, Vec<EventGroup>) {
    let mut state = state;
    state.remove_operator(&ctx.sender, &msg.operator);

    (state, vec![])
}
