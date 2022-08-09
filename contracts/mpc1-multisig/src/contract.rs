use crate::state::ContractState;

use pbc_contract_common::{context::ContractContext, events::EventGroup};

use mpc1_multisig_base::{
    actions::{
        execute_close_proposal, execute_create_proposal, execute_execute_proposal, execute_init,
        execute_vote,
    },
    msg::{CreateProposalMsg, InitMsg, ProposalCloseMsg, ProposalExecuteMsg, ProposalVoteMsg},
};

#[init]
pub fn initialize(ctx: ContractContext, msg: InitMsg) -> (ContractState, Vec<EventGroup>) {
    let (mpc1_multisig, events) = execute_init(&ctx, &msg);
    let state = ContractState { mpc1_multisig };

    (state, events)
}

#[action]
pub fn create_proposal(
    ctx: ContractContext,
    state: ContractState,
    msg: CreateProposalMsg,
) -> (ContractState, Vec<EventGroup>) {
    let mut state = state;
    let events = execute_create_proposal(&ctx, &mut state.mpc1_multisig, &msg);

    (state, events)
}

#[action]
pub fn vote(
    ctx: ContractContext,
    state: ContractState,
    msg: ProposalVoteMsg,
) -> (ContractState, Vec<EventGroup>) {
    let mut state = state;
    let events = execute_vote(&ctx, &mut state.mpc1_multisig, &msg);

    (state, events)
}

#[action]
pub fn execute_proposal(
    ctx: ContractContext,
    state: ContractState,
    msg: ProposalExecuteMsg,
) -> (ContractState, Vec<EventGroup>) {
    let mut state = state;
    let events = execute_execute_proposal(&ctx, &mut state.mpc1_multisig, &msg);

    (state, events)
}

#[action]
pub fn close_proposal(
    ctx: ContractContext,
    state: ContractState,
    msg: ProposalCloseMsg,
) -> (ContractState, Vec<EventGroup>) {
    let mut state = state;
    let events = execute_close_proposal(&ctx, &mut state.mpc1_multisig, &msg);

    (state, events)
}
