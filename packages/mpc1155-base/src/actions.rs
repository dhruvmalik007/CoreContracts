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
    _ctx: &ContractContext,
    msg: &InitMsg,
) -> (MPC1155ContractState, Vec<EventGroup>) {
    let state = MPC1155ContractState {
        owner: msg.owner,
        uri: msg.uri.clone(),
        minter: msg.minter,
        balances: BTreeMap::new(),
        operator_approvals: BTreeMap::new(),
        tokens: BTreeMap::new(),
    };

    (state, vec![])
}

pub fn execute_set_uri(
    ctx: &ContractContext,
    state: &mut MPC1155ContractState,
    msg: &SetUriMsg,
) -> Vec<EventGroup> {
    assert!(
        state.is_owner(&ctx.sender),
        "{}",
        ContractError::Unauthorized
    );

    state.set_uri(&msg.new_uri);
    vec![]
}

pub fn execute_mint(
    ctx: &ContractContext,
    state: &mut MPC1155ContractState,
    msg: &MintMsg,
) -> Vec<EventGroup> {
    assert!(
        state.minter == ctx.sender,
        "{}",
        ContractError::Unauthorized
    );

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

    vec![]
}

pub fn execute_batch_mint(
    ctx: &ContractContext,
    state: &mut MPC1155ContractState,
    msg: &BatchMintMsg,
) -> Vec<EventGroup> {
    assert!(
        state.minter == ctx.sender,
        "{}",
        ContractError::Unauthorized
    );

    for token_info in msg.token_infos.iter() {
        state.store_token(
            token_info.token_id,
            &TokenInfo {
                token_uri: token_info.token_uri.clone(),
            },
        );
        state.transfer(None, Some(&msg.to), token_info.token_id, token_info.amount);
    }

    vec![]
}

pub fn execute_transfer_from(
    ctx: &ContractContext,
    state: &mut MPC1155ContractState,
    msg: &TransferFromMsg,
) -> Vec<EventGroup> {
    assert!(
        state.is_token_owner_or_operator(&msg.from, &ctx.sender),
        "{}",
        ContractError::Unauthorized,
    );

    state.transfer(
        Some(&msg.from),
        Some(&msg.to),
        msg.token_info.token_id,
        msg.token_info.amount,
    );

    vec![]
}

pub fn execute_batch_transfer_from(
    ctx: &ContractContext,
    state: &mut MPC1155ContractState,
    msg: &BatchTransferFromMsg,
) -> Vec<EventGroup> {
    assert!(
        state.is_token_owner_or_operator(&msg.from, &ctx.sender),
        "{}",
        ContractError::Unauthorized,
    );

    for token_info in msg.token_infos.iter() {
        state.transfer(
            Some(&msg.from),
            Some(&msg.to),
            token_info.token_id,
            token_info.amount,
        );
    }

    vec![]
}

pub fn execute_burn(
    ctx: &ContractContext,
    state: &mut MPC1155ContractState,
    msg: &BurnMsg,
) -> Vec<EventGroup> {
    assert!(
        state.is_token_owner_or_operator(&msg.from, &ctx.sender),
        "{}",
        ContractError::Unauthorized,
    );

    state.transfer(
        Some(&msg.from),
        None,
        msg.token_info.token_id,
        msg.token_info.amount,
    );

    vec![]
}

pub fn execute_batch_burn(
    ctx: &ContractContext,
    state: &mut MPC1155ContractState,
    msg: &BatchBurnMsg,
) -> Vec<EventGroup> {
    assert!(
        state.is_token_owner_or_operator(&msg.from, &ctx.sender),
        "{}",
        ContractError::Unauthorized,
    );

    for token_info in msg.token_infos.iter() {
        state.transfer(
            Some(&msg.from),
            None,
            token_info.token_id,
            token_info.amount,
        );
    }

    vec![]
}

pub fn execute_approve_for_all(
    ctx: &ContractContext,
    state: &mut MPC1155ContractState,
    msg: &ApproveForAllMsg,
) -> Vec<EventGroup> {
    state.add_operator(&ctx.sender, &msg.operator);
    vec![]
}

pub fn execute_revoke_for_all(
    ctx: &ContractContext,
    state: &mut MPC1155ContractState,
    msg: &RevokeForAllMsg,
) -> Vec<EventGroup> {
    state.remove_operator(&ctx.sender, &msg.operator);
    vec![]
}
