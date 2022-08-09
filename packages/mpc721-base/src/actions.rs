use std::collections::BTreeMap;

use pbc_contract_common::{context::ContractContext, events::EventGroup};

use crate::{
    msg::{
        ApproveForAllMsg, ApproveMsg, BurnMsg, InitMsg, MintMsg, RevokeForAllMsg, RevokeMsg,
        SetBaseUriMsg, TransferFromMsg, TransferMsg,
    },
    state::MPC721ContractState,
    ContractError,
};

pub fn execute_init(
    _ctx: &ContractContext,
    msg: &InitMsg,
) -> (MPC721ContractState, Vec<EventGroup>) {
    let state = MPC721ContractState {
        owner: msg.owner,
        name: msg.name.clone(),
        symbol: msg.symbol.clone(),
        base_uri: msg.base_uri.clone(),
        minter: msg.minter,
        supply: 0,
        tokens: BTreeMap::new(),
        operator_approvals: BTreeMap::new(),
    };

    (state, vec![])
}

pub fn execute_set_base_uri(
    ctx: &ContractContext,
    state: &mut MPC721ContractState,
    msg: &SetBaseUriMsg,
) -> Vec<EventGroup> {
    assert!(
        state.is_owner(&ctx.sender),
        "{}",
        ContractError::Unauthorized
    );

    state.set_base_uri(&msg.new_base_uri);
    vec![]
}

pub fn execute_mint(
    ctx: &ContractContext,
    state: &mut MPC721ContractState,
    msg: &MintMsg,
) -> Vec<EventGroup> {
    assert!(
        state.minter == ctx.sender,
        "{}",
        ContractError::Unauthorized
    );

    assert!(!state.is_minted(msg.token_id), "{}", ContractError::Minted);

    state.mint(msg.token_id, &msg.to, &msg.token_uri);
    state.increase_supply();

    vec![]
}

pub fn execute_transfer(
    ctx: &ContractContext,
    state: &mut MPC721ContractState,
    msg: &TransferMsg,
) -> Vec<EventGroup> {
    assert!(state.is_minted(msg.token_id), "{}", ContractError::NotFound);

    state.transfer(&ctx.sender, &msg.to, msg.token_id);
    vec![]
}

pub fn execute_transfer_from(
    ctx: &ContractContext,
    state: &mut MPC721ContractState,
    msg: &TransferFromMsg,
) -> Vec<EventGroup> {
    assert!(state.is_minted(msg.token_id), "{}", ContractError::NotFound);

    state.transfer(&msg.from, &msg.to, msg.token_id);
    vec![]
}

pub fn execute_approve(
    ctx: &ContractContext,
    state: &mut MPC721ContractState,
    msg: &ApproveMsg,
) -> Vec<EventGroup> {
    assert!(state.is_minted(msg.token_id), "{}", ContractError::NotFound);

    state.update_approvals(&ctx.sender, &msg.spender, msg.token_id, true);
    vec![]
}

pub fn execute_approve_for_all(
    ctx: &ContractContext,
    state: &mut MPC721ContractState,
    msg: &ApproveForAllMsg,
) -> Vec<EventGroup> {
    state.add_operator(&ctx.sender, &msg.operator);
    vec![]
}

pub fn execute_revoke(
    ctx: &ContractContext,
    state: &mut MPC721ContractState,
    msg: &RevokeMsg,
) -> Vec<EventGroup> {
    assert!(state.is_minted(msg.token_id), "{}", ContractError::NotFound);

    state.update_approvals(&ctx.sender, &msg.spender, msg.token_id, false);
    vec![]
}

pub fn execute_revoke_for_all(
    ctx: &ContractContext,
    state: &mut MPC721ContractState,
    msg: &RevokeForAllMsg,
) -> Vec<EventGroup> {
    state.remove_operator(&ctx.sender, &msg.operator);
    vec![]
}

pub fn execute_burn(
    ctx: &ContractContext,
    state: &mut MPC721ContractState,
    msg: &BurnMsg,
) -> Vec<EventGroup> {
    assert!(state.is_minted(msg.token_id), "{}", ContractError::NotFound);

    state.remove_token(&ctx.sender, msg.token_id);
    state.decrease_supply();

    vec![]
}
