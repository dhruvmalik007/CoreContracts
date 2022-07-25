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

pub fn execute_init(_ctx: ContractContext, msg: InitMsg) -> (MPC721ContractState, Vec<EventGroup>) {
    let state = MPC721ContractState {
        owner: msg.owner,
        name: msg.name,
        symbol: msg.symbol,
        base_uri: msg.base_uri,
        minter: msg.minter,
        supply: 0,
        tokens: BTreeMap::new(),
        operator_approvals: BTreeMap::new(),
    };

    (state, vec![])
}

pub fn execute_set_base_uri(
    ctx: ContractContext,
    state: MPC721ContractState,
    msg: SetBaseUriMsg,
) -> (MPC721ContractState, Vec<EventGroup>) {
    assert!(
        state.is_owner(&ctx.sender),
        "{}",
        ContractError::Unauthorized
    );

    let mut state = state;
    state.set_base_uri(&msg.new_base_uri);

    (state, vec![])
}

pub fn execute_mint(
    ctx: ContractContext,
    state: MPC721ContractState,
    msg: MintMsg,
) -> (MPC721ContractState, Vec<EventGroup>) {
    assert!(
        state.minter == ctx.sender,
        "{}",
        ContractError::Unauthorized
    );

    assert!(!state.is_minted(msg.token_id), "{}", ContractError::Minted);

    let mut state = state;
    state.mint(msg.token_id, &msg.to, &msg.token_uri);
    state.increase_supply();

    (state, vec![])
}

pub fn execute_transfer(
    ctx: ContractContext,
    state: MPC721ContractState,
    msg: TransferMsg,
) -> (MPC721ContractState, Vec<EventGroup>) {
    assert!(state.is_minted(msg.token_id), "{}", ContractError::NotFound);

    let mut state = state;
    state.transfer(&ctx.sender, &msg.to, msg.token_id);

    (state, vec![])
}

pub fn execute_transfer_from(
    ctx: ContractContext,
    state: MPC721ContractState,
    msg: TransferFromMsg,
) -> (MPC721ContractState, Vec<EventGroup>) {
    assert!(state.is_minted(msg.token_id), "{}", ContractError::NotFound);

    let mut state = state;
    state.transfer(&msg.from, &msg.to, msg.token_id);

    (state, vec![])
}

pub fn execute_approve(
    ctx: ContractContext,
    state: MPC721ContractState,
    msg: ApproveMsg,
) -> (MPC721ContractState, Vec<EventGroup>) {
    assert!(state.is_minted(msg.token_id), "{}", ContractError::NotFound);

    let mut state = state;
    state.update_approvals(&ctx.sender, &msg.spender, msg.token_id, true);

    (state, vec![])
}

pub fn execute_approve_for_all(
    ctx: ContractContext,
    state: MPC721ContractState,
    msg: ApproveForAllMsg,
) -> (MPC721ContractState, Vec<EventGroup>) {
    let mut state = state;
    state.add_operator(&ctx.sender, &msg.operator);

    (state, vec![])
}

pub fn execute_revoke(
    ctx: ContractContext,
    state: MPC721ContractState,
    msg: RevokeMsg,
) -> (MPC721ContractState, Vec<EventGroup>) {
    assert!(state.is_minted(msg.token_id), "{}", ContractError::NotFound);

    let mut state = state;
    state.update_approvals(&ctx.sender, &msg.spender, msg.token_id, false);

    (state, vec![])
}

pub fn execute_revoke_for_all(
    ctx: ContractContext,
    state: MPC721ContractState,
    msg: RevokeForAllMsg,
) -> (MPC721ContractState, Vec<EventGroup>) {
    let mut state = state;
    state.remove_operator(&ctx.sender, &msg.operator);

    (state, vec![])
}

pub fn execute_burn(
    ctx: ContractContext,
    state: MPC721ContractState,
    msg: BurnMsg,
) -> (MPC721ContractState, Vec<EventGroup>) {
    assert!(state.is_minted(msg.token_id), "{}", ContractError::NotFound);

    let mut state = state;
    state.remove_token(&ctx.sender, msg.token_id);
    state.decrease_supply();

    (state, vec![])
}
