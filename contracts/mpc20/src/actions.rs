use pbc_contract_common::{context::ContractContext, events::EventGroup};

use crate::{
    msg::{
        ApproveMsg, BurnFromMsg, BurnMsg, DecreaseAllowanceMsg, IncreaseAllowanceMsg, MintMsg,
        TransferFromMsg, TransferMsg,
    },
    state::MPC20ContractState,
    ContractError,
};

pub fn execute_mint(
    ctx: ContractContext,
    state: MPC20ContractState,
    msg: MintMsg,
) -> (MPC20ContractState, Vec<EventGroup>) {
    assert!(
        msg.amount > 0,
        "{}",
        ContractError::AmountMustBeHigherThenZero,
    );

    let mut state = state;
    assert!(
        state.minter.is_some(),
        "{}",
        ContractError::MintingIsDisabled
    );
    assert!(
        state.minter.as_ref().unwrap().minter == ctx.sender,
        "{}",
        ContractError::Unauthorized
    );

    state.increase_total_supply(msg.amount);
    if let Some(limit) = state.get_capacity() {
        assert!(
            state.total_supply <= limit,
            "{}",
            ContractError::CapacityExceeded
        );
    }

    state.increase_balance(&msg.recipient, msg.amount);

    (state, vec![])
}

pub fn execute_transfer(
    ctx: ContractContext,
    state: MPC20ContractState,
    msg: TransferMsg,
) -> (MPC20ContractState, Vec<EventGroup>) {
    assert!(
        msg.amount > 0,
        "{}",
        ContractError::AmountMustBeHigherThenZero,
    );

    let mut state = state;
    state.decrease_balance(&ctx.sender, msg.amount);
    state.increase_balance(&msg.to, msg.amount);

    (state, vec![])
}

pub fn execute_transfer_from(
    _ctx: ContractContext,
    state: MPC20ContractState,
    msg: TransferFromMsg,
) -> (MPC20ContractState, Vec<EventGroup>) {
    assert!(
        msg.amount > 0,
        "{}",
        ContractError::AmountMustBeHigherThenZero,
    );

    let mut state = state;

    state.decrease_allowance(&msg.owner, &msg.to, msg.amount);
    state.decrease_balance(&msg.owner, msg.amount);
    state.increase_balance(&msg.to, msg.amount);

    (state, vec![])
}

pub fn execute_burn(
    ctx: ContractContext,
    state: MPC20ContractState,
    msg: BurnMsg,
) -> (MPC20ContractState, Vec<EventGroup>) {
    assert!(
        msg.amount > 0,
        "{}",
        ContractError::AmountMustBeHigherThenZero,
    );

    let mut state = state;
    state.decrease_balance(&ctx.sender, msg.amount);
    state.decrease_total_supply(msg.amount);

    (state, vec![])
}

pub fn execute_burn_from(
    ctx: ContractContext,
    state: MPC20ContractState,
    msg: BurnFromMsg,
) -> (MPC20ContractState, Vec<EventGroup>) {
    assert!(
        msg.amount > 0,
        "{}",
        ContractError::AmountMustBeHigherThenZero,
    );

    let mut state = state;

    state.decrease_allowance(&msg.owner, &ctx.sender, msg.amount);
    state.decrease_balance(&msg.owner, msg.amount);
    state.decrease_total_supply(msg.amount);

    (state, vec![])
}

pub fn execute_approve(
    ctx: ContractContext,
    state: MPC20ContractState,
    msg: ApproveMsg,
) -> (MPC20ContractState, Vec<EventGroup>) {
    assert!(
        ctx.sender != msg.spender,
        "{}",
        ContractError::CannotApproveToYourself
    );

    assert!(
        msg.amount > 0,
        "{}",
        ContractError::AmountMustBeHigherThenZero,
    );

    let mut state = state;
    state.set_allowance(&ctx.sender, &msg.spender, msg.amount);

    (state, vec![])
}

pub fn execute_increase_allowance(
    ctx: ContractContext,
    state: MPC20ContractState,
    msg: IncreaseAllowanceMsg,
) -> (MPC20ContractState, Vec<EventGroup>) {
    assert!(
        ctx.sender != msg.spender,
        "{}",
        ContractError::CannotApproveToYourself
    );

    assert!(
        msg.amount > 0,
        "{}",
        ContractError::AmountMustBeHigherThenZero,
    );

    let mut state = state;
    state.increase_allowance(&ctx.sender, &msg.spender, msg.amount);

    (state, vec![])
}

pub fn execute_decrease_allowance(
    ctx: ContractContext,
    state: MPC20ContractState,
    msg: DecreaseAllowanceMsg,
) -> (MPC20ContractState, Vec<EventGroup>) {
    assert!(
        ctx.sender != msg.spender,
        "{}",
        ContractError::CannotApproveToYourself
    );

    assert!(
        msg.amount > 0,
        "{}",
        ContractError::AmountMustBeHigherThenZero,
    );

    let mut state = state;
    state.decrease_allowance(&ctx.sender, &msg.spender, msg.amount);

    (state, vec![])
}
