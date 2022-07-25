use std::collections::BTreeMap;

use mpc20::{
    msg::TransferMsg,
    state::{MPC20ContractState, Minter, TokenInfo},
};
use pbc_contract_common::{
    address::{Address, AddressType},
    context::ContractContext,
    events::EventGroup,
};
use utils::{decimal::DecimalRatio, events::into_rpc_call};

use crate::{
    contract::{claim, compound, initialize, stake, unstake},
    msg::{ClaimMsg, CompoundMsg, InitMsg, StakeMsg, UnstakeMsg},
    state::{MPC20StakingContractState, Staker},
};

fn mock_address(le: u8) -> Address {
    Address {
        address_type: AddressType::Account,
        identifier: [
            le, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
            0u8, 0u8, 0u8,
        ],
    }
}

fn mock_contract_context(sender: u8, block_time: i64) -> ContractContext {
    ContractContext {
        contract_address: mock_address(1u8),
        sender: mock_address(sender),
        block_time,
        block_production_time: 10,
        current_transaction: [
            0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
            0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
        ],
        original_transaction: [
            0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
            0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
        ],
    }
}

#[test]
fn test_staking() {
    const DEPOSIT_TOKEN: u8 = 1;
    const MINTER: u8 = 9;
    const ALICE: u8 = 10;
    const BOB: u8 = 11;
    const JACK: u8 = 12;

    let mut block_time = 100;

    let msg = InitMsg {
        deposit_token: None,
        distribution_amount: 1_000,
        distribution_epoch: 10,
        compound_frequency: 100,
        info: TokenInfo {
            name: "Staking Token".to_string(),
            symbol: "STKN".to_string(),
            decimals: 18,
        },
        initial_balances: vec![],
        minter: Some(mock_address(MINTER)),
    };
    let (state, events) = initialize(mock_contract_context(MINTER, block_time), msg);
    assert_eq!(events, vec![]);
    assert_eq!(
        state,
        MPC20StakingContractState {
            deposit_token: mock_address(DEPOSIT_TOKEN),
            distribution_amount: 1_000,
            distribution_epoch: 10,
            global_index: DecimalRatio::new(0, 0),
            total_staked: 0,
            last_distributed: 100,
            stakers: BTreeMap::new(),
            compound_frequency: 100,
            mpc20_base_state: MPC20ContractState {
                info: TokenInfo {
                    name: "Staking Token".to_string(),
                    symbol: "STKN".to_string(),
                    decimals: 18,
                },
                total_supply: 0,
                minter: Some(Minter {
                    minter: mock_address(MINTER),
                    capacity: None,
                }),
                balances: BTreeMap::new(),
                allowances: BTreeMap::new(),
            },
        }
    );

    // no distribution yet, total_stake is zero
    block_time = 105;

    let msg = StakeMsg { amount: 100 };
    let (state, events) = stake(mock_contract_context(ALICE, block_time), state, msg);

    assert_eq!(events.len(), 1);
    let mut eg = EventGroup::new();
    eg.send_from_original_sender(
        &mock_address(DEPOSIT_TOKEN),
        into_rpc_call(TransferMsg {
            to: mock_address(DEPOSIT_TOKEN),
            amount: 100,
        }),
        None,
    );
    assert_eq!(events[0], eg);

    assert_eq!(
        state,
        MPC20StakingContractState {
            deposit_token: mock_address(DEPOSIT_TOKEN),
            distribution_amount: 1_000,
            distribution_epoch: 10,
            global_index: DecimalRatio::new(0, 0),
            total_staked: 100,
            last_distributed: 105,
            stakers: BTreeMap::from([(
                mock_address(ALICE),
                Staker {
                    reward_index: DecimalRatio::zero(),
                    staked_amount: 100,
                    pending_reward: 0,
                    last_compound: 0,
                }
            )]),
            compound_frequency: 100,
            mpc20_base_state: MPC20ContractState {
                info: TokenInfo {
                    name: "Staking Token".to_string(),
                    symbol: "STKN".to_string(),
                    decimals: 18,
                },
                total_supply: 0,
                minter: Some(Minter {
                    minter: mock_address(MINTER),
                    capacity: None,
                }),
                balances: BTreeMap::new(),
                allowances: BTreeMap::new(),
            },
        }
    );

    // no distribution yet, total stake 100
    block_time = 114;

    let msg = StakeMsg { amount: 100 };
    let (state, events) = stake(mock_contract_context(BOB, block_time), state, msg);
    assert_eq!(
        state,
        MPC20StakingContractState {
            deposit_token: mock_address(DEPOSIT_TOKEN),
            distribution_amount: 1_000,
            distribution_epoch: 10,
            global_index: DecimalRatio::new(0, 0),
            total_staked: 200,
            last_distributed: 105,
            stakers: BTreeMap::from([
                (
                    mock_address(ALICE),
                    Staker {
                        reward_index: DecimalRatio::zero(),
                        staked_amount: 100,
                        pending_reward: 0,
                        last_compound: 0,
                    }
                ),
                (
                    mock_address(BOB),
                    Staker {
                        reward_index: DecimalRatio::zero(),
                        staked_amount: 100,
                        pending_reward: 0,
                        last_compound: 0,
                    }
                )
            ]),
            compound_frequency: 100,
            mpc20_base_state: MPC20ContractState {
                info: TokenInfo {
                    name: "Staking Token".to_string(),
                    symbol: "STKN".to_string(),
                    decimals: 18,
                },
                total_supply: 0,
                minter: Some(Minter {
                    minter: mock_address(MINTER),
                    capacity: None,
                }),
                balances: BTreeMap::new(),
                allowances: BTreeMap::new(),
            },
        }
    );

    // first distribution, ALICE and BOB must claim and receive equal rewards
    block_time = 115;

    let msg = ClaimMsg { amount: None };
    let (state, _) = claim(mock_contract_context(ALICE, block_time), state, msg);
    assert_eq!(
        state,
        MPC20StakingContractState {
            deposit_token: mock_address(DEPOSIT_TOKEN),
            distribution_amount: 1_000,
            distribution_epoch: 10,
            global_index: DecimalRatio::new(5, 0),
            total_staked: 200,
            last_distributed: 115,
            stakers: BTreeMap::from([
                (
                    mock_address(ALICE),
                    Staker {
                        reward_index: DecimalRatio::new(5, 0),
                        staked_amount: 100,
                        pending_reward: 0, // pending reward claimed(minted)
                        last_compound: 0,
                    }
                ),
                (
                    mock_address(BOB),
                    Staker {
                        reward_index: DecimalRatio::zero(),
                        staked_amount: 100,
                        pending_reward: 0,
                        last_compound: 0,
                    }
                )
            ]),
            compound_frequency: 100,
            mpc20_base_state: MPC20ContractState {
                info: TokenInfo {
                    name: "Staking Token".to_string(),
                    symbol: "STKN".to_string(),
                    decimals: 18,
                },
                total_supply: 500,
                minter: Some(Minter {
                    minter: mock_address(MINTER),
                    capacity: None,
                }),
                balances: BTreeMap::from([(mock_address(ALICE), 500),]),
                allowances: BTreeMap::new(),
            },
        }
    );

    block_time = 116;
    let msg = ClaimMsg { amount: None };
    let (state, _) = claim(mock_contract_context(BOB, block_time), state, msg);
    assert_eq!(
        state,
        MPC20StakingContractState {
            deposit_token: mock_address(DEPOSIT_TOKEN),
            distribution_amount: 1_000,
            distribution_epoch: 10,
            global_index: DecimalRatio::new(5, 0),
            total_staked: 200,
            last_distributed: 115,
            stakers: BTreeMap::from([
                (
                    mock_address(ALICE),
                    Staker {
                        reward_index: DecimalRatio::new(5, 0),
                        staked_amount: 100,
                        pending_reward: 0, // pending reward claimed(minted)
                        last_compound: 0,
                    }
                ),
                (
                    mock_address(BOB),
                    Staker {
                        reward_index: DecimalRatio::new(5, 0),
                        staked_amount: 100,
                        pending_reward: 0,
                        last_compound: 0,
                    }
                )
            ]),
            compound_frequency: 100,
            mpc20_base_state: MPC20ContractState {
                info: TokenInfo {
                    name: "Staking Token".to_string(),
                    symbol: "STKN".to_string(),
                    decimals: 18,
                },
                total_supply: 1_000,
                minter: Some(Minter {
                    minter: mock_address(MINTER),
                    capacity: None,
                }),
                balances: BTreeMap::from([(mock_address(ALICE), 500), (mock_address(BOB), 500)]),
                allowances: BTreeMap::new(),
            },
        }
    );

    // BOB unstakes half
    block_time = 120;
    let msg = UnstakeMsg { amount: 50 };
    let (state, _) = unstake(mock_contract_context(BOB, block_time), state, msg);
    assert_eq!(
        state,
        MPC20StakingContractState {
            deposit_token: mock_address(DEPOSIT_TOKEN),
            distribution_amount: 1_000,
            distribution_epoch: 10,
            global_index: DecimalRatio::new(5, 0),
            total_staked: 150,
            last_distributed: 115,
            stakers: BTreeMap::from([
                (
                    mock_address(ALICE),
                    Staker {
                        reward_index: DecimalRatio::new(5, 0),
                        staked_amount: 100,
                        pending_reward: 0, // pending reward claimed(minted)
                        last_compound: 0,
                    }
                ),
                (
                    mock_address(BOB),
                    Staker {
                        reward_index: DecimalRatio::new(5, 0),
                        staked_amount: 50,
                        pending_reward: 0,
                        last_compound: 0,
                    }
                )
            ]),
            compound_frequency: 100,
            mpc20_base_state: MPC20ContractState {
                info: TokenInfo {
                    name: "Staking Token".to_string(),
                    symbol: "STKN".to_string(),
                    decimals: 18,
                },
                total_supply: 1_000,
                minter: Some(Minter {
                    minter: mock_address(MINTER),
                    capacity: None,
                }),
                balances: BTreeMap::from([(mock_address(ALICE), 500), (mock_address(BOB), 500)]),
                allowances: BTreeMap::new(),
            },
        }
    );

    // next distribution, ALICE share 66.7, BOB share 33.3
    block_time = 125;
    let msg = ClaimMsg { amount: Some(100) };
    let (state, _) = claim(mock_contract_context(ALICE, block_time), state, msg);
    let msg = ClaimMsg { amount: None };
    let (state, _) = claim(mock_contract_context(BOB, block_time), state, msg);
    assert_eq!(
        state,
        MPC20StakingContractState {
            deposit_token: mock_address(DEPOSIT_TOKEN),
            distribution_amount: 1_000,
            distribution_epoch: 10,
            global_index: DecimalRatio::new(11666666666666666666666666667, 27),
            total_staked: 150,
            last_distributed: 125,
            stakers: BTreeMap::from([
                (
                    mock_address(ALICE),
                    Staker {
                        reward_index: DecimalRatio::new(11666666666666666666666666667, 27),
                        staked_amount: 100,
                        pending_reward: 566, // 666 - claim_amount
                        last_compound: 0,
                    }
                ),
                (
                    mock_address(BOB),
                    Staker {
                        reward_index: DecimalRatio::new(11666666666666666666666666667, 27),
                        staked_amount: 50,
                        pending_reward: 0,
                        last_compound: 0,
                    }
                )
            ]),
            compound_frequency: 100,
            mpc20_base_state: MPC20ContractState {
                info: TokenInfo {
                    name: "Staking Token".to_string(),
                    symbol: "STKN".to_string(),
                    decimals: 18,
                },
                total_supply: 1_433,
                minter: Some(Minter {
                    minter: mock_address(MINTER),
                    capacity: None,
                }),
                balances: BTreeMap::from([(mock_address(ALICE), 600), (mock_address(BOB), 833)]),
                allowances: BTreeMap::new(),
            },
        }
    );

    // JACK stakes 50, ALICE share 50, BOB and JACK 25 resp
    block_time = 134;

    let msg = StakeMsg { amount: 50 };
    let (state, events) = stake(mock_contract_context(JACK, block_time), state, msg);
    assert_eq!(
        state,
        MPC20StakingContractState {
            deposit_token: mock_address(DEPOSIT_TOKEN),
            distribution_amount: 1_000,
            distribution_epoch: 10,
            global_index: DecimalRatio::new(11666666666666666666666666667, 27),
            total_staked: 200,
            last_distributed: 125,
            stakers: BTreeMap::from([
                (
                    mock_address(ALICE),
                    Staker {
                        reward_index: DecimalRatio::new(11666666666666666666666666667, 27),
                        staked_amount: 100,
                        pending_reward: 566, // 666 - claim_amount
                        last_compound: 0,
                    }
                ),
                (
                    mock_address(BOB),
                    Staker {
                        reward_index: DecimalRatio::new(11666666666666666666666666667, 27),
                        staked_amount: 50,
                        pending_reward: 0,
                        last_compound: 0,
                    }
                ),
                (
                    mock_address(JACK),
                    Staker {
                        reward_index: DecimalRatio::new(11666666666666666666666666667, 27),
                        staked_amount: 50,
                        pending_reward: 0,
                        last_compound: 0,
                    }
                )
            ]),
            compound_frequency: 100,
            mpc20_base_state: MPC20ContractState {
                info: TokenInfo {
                    name: "Staking Token".to_string(),
                    symbol: "STKN".to_string(),
                    decimals: 18,
                },
                total_supply: 1_433,
                minter: Some(Minter {
                    minter: mock_address(MINTER),
                    capacity: None,
                }),
                balances: BTreeMap::from([(mock_address(ALICE), 600), (mock_address(BOB), 833)]),
                allowances: BTreeMap::new(),
            },
        }
    );

    // everyone claims 1 token
    block_time = 140;

    let msg = ClaimMsg { amount: Some(1) };
    let (state, _) = claim(mock_contract_context(ALICE, block_time), state, msg.clone());
    let (state, _) = claim(mock_contract_context(BOB, block_time), state, msg.clone());
    let (state, _) = claim(mock_contract_context(JACK, block_time), state, msg.clone());
    assert_eq!(
        state,
        MPC20StakingContractState {
            deposit_token: mock_address(DEPOSIT_TOKEN),
            distribution_amount: 1_000,
            distribution_epoch: 10,
            global_index: DecimalRatio::new(16666666666666666666666666667, 27),
            total_staked: 200,
            last_distributed: 135,
            stakers: BTreeMap::from([
                (
                    mock_address(ALICE),
                    Staker {
                        reward_index: DecimalRatio::new(16666666666666666666666666667, 27),
                        staked_amount: 100,
                        pending_reward: 1065,
                        last_compound: 0,
                    }
                ),
                (
                    mock_address(BOB),
                    Staker {
                        reward_index: DecimalRatio::new(16666666666666666666666666667, 27),
                        staked_amount: 50,
                        pending_reward: 249,
                        last_compound: 0,
                    }
                ),
                (
                    mock_address(JACK),
                    Staker {
                        reward_index: DecimalRatio::new(16666666666666666666666666667, 27),
                        staked_amount: 50,
                        pending_reward: 249,
                        last_compound: 0,
                    }
                )
            ]),
            compound_frequency: 100,
            mpc20_base_state: MPC20ContractState {
                info: TokenInfo {
                    name: "Staking Token".to_string(),
                    symbol: "STKN".to_string(),
                    decimals: 18,
                },
                total_supply: 1_436,
                minter: Some(Minter {
                    minter: mock_address(MINTER),
                    capacity: None,
                }),
                balances: BTreeMap::from([
                    (mock_address(ALICE), 601),
                    (mock_address(BOB), 834),
                    (mock_address(JACK), 1)
                ]),
                allowances: BTreeMap::new(),
            },
        }
    );

    // JACK compounds
    block_time = 144;

    let msg = CompoundMsg { amount: Some(100) };
    let (state, events) = compound(mock_contract_context(JACK, block_time), state, msg);
    assert_eq!(
        state,
        MPC20StakingContractState {
            deposit_token: mock_address(DEPOSIT_TOKEN),
            distribution_amount: 1_000,
            distribution_epoch: 10,
            global_index: DecimalRatio::new(16666666666666666666666666667, 27),
            total_staked: 300,
            last_distributed: 135,
            stakers: BTreeMap::from([
                (
                    mock_address(ALICE),
                    Staker {
                        reward_index: DecimalRatio::new(16666666666666666666666666667, 27),
                        staked_amount: 100,
                        pending_reward: 1065,
                        last_compound: 0,
                    }
                ),
                (
                    mock_address(BOB),
                    Staker {
                        reward_index: DecimalRatio::new(16666666666666666666666666667, 27),
                        staked_amount: 50,
                        pending_reward: 249,
                        last_compound: 0,
                    }
                ),
                (
                    mock_address(JACK),
                    Staker {
                        reward_index: DecimalRatio::new(16666666666666666666666666667, 27),
                        staked_amount: 150,
                        pending_reward: 149,
                        last_compound: 144,
                    }
                )
            ]),
            compound_frequency: 100,
            mpc20_base_state: MPC20ContractState {
                info: TokenInfo {
                    name: "Staking Token".to_string(),
                    symbol: "STKN".to_string(),
                    decimals: 18,
                },
                total_supply: 1_536,
                minter: Some(Minter {
                    minter: mock_address(MINTER),
                    capacity: None,
                }),
                balances: BTreeMap::from([
                    (mock_address(ALICE), 601),
                    (mock_address(BOB), 834),
                    (mock_address(JACK), 1),
                    (mock_address(DEPOSIT_TOKEN), 100) // compound - claim + stake
                ]),
                allowances: BTreeMap::new(),
            },
        }
    );
}

#[test]
#[should_panic(expected = "Distribution amount must be higher then 0")]
fn invalid_distribution_amount() {
    const MINTER: u8 = 9;

    let block_time = 100;

    let msg = InitMsg {
        deposit_token: None,
        distribution_amount: 0,
        distribution_epoch: 10,
        compound_frequency: 100,
        info: TokenInfo {
            name: "Staking Token".to_string(),
            symbol: "STKN".to_string(),
            decimals: 18,
        },
        initial_balances: vec![],
        minter: Some(mock_address(MINTER)),
    };
    let (_, _) = initialize(mock_contract_context(MINTER, block_time), msg);
}

#[test]
#[should_panic(expected = "Distribution epoch must be higher then 0")]
fn invalid_distribution_epoch() {
    const MINTER: u8 = 9;

    let block_time = 100;

    let msg = InitMsg {
        deposit_token: None,
        distribution_amount: 1_000,
        distribution_epoch: 0,
        compound_frequency: 100,
        info: TokenInfo {
            name: "Staking Token".to_string(),
            symbol: "STKN".to_string(),
            decimals: 18,
        },
        initial_balances: vec![],
        minter: Some(mock_address(MINTER)),
    };
    let (_, _) = initialize(mock_contract_context(MINTER, block_time), msg);
}

#[test]
#[should_panic(expected = "Cannot unstake more then staked")]
fn unstake_more_then_staked() {
    const DEPOSIT_TOKEN: u8 = 1;
    const MINTER: u8 = 9;
    const ALICE: u8 = 10;
    const BOB: u8 = 11;
    const JACK: u8 = 12;

    let block_time = 150;

    let state = MPC20StakingContractState {
        deposit_token: mock_address(DEPOSIT_TOKEN),
        distribution_amount: 1_000,
        distribution_epoch: 10,
        global_index: DecimalRatio::new(16666666666666666666666666667, 27),
        total_staked: 300,
        last_distributed: 135,
        stakers: BTreeMap::from([
            (
                mock_address(ALICE),
                Staker {
                    reward_index: DecimalRatio::new(16666666666666666666666666667, 27),
                    staked_amount: 100,
                    pending_reward: 1065,
                    last_compound: 0,
                },
            ),
            (
                mock_address(BOB),
                Staker {
                    reward_index: DecimalRatio::new(16666666666666666666666666667, 27),
                    staked_amount: 50,
                    pending_reward: 249,
                    last_compound: 0,
                },
            ),
            (
                mock_address(JACK),
                Staker {
                    reward_index: DecimalRatio::new(16666666666666666666666666667, 27),
                    staked_amount: 150,
                    pending_reward: 149,
                    last_compound: 144,
                },
            ),
        ]),
        compound_frequency: 100,
        mpc20_base_state: MPC20ContractState {
            info: TokenInfo {
                name: "Staking Token".to_string(),
                symbol: "STKN".to_string(),
                decimals: 18,
            },
            total_supply: 1_536,
            minter: Some(Minter {
                minter: mock_address(MINTER),
                capacity: None,
            }),
            balances: BTreeMap::from([
                (mock_address(ALICE), 601),
                (mock_address(BOB), 834),
                (mock_address(JACK), 1),
                (mock_address(DEPOSIT_TOKEN), 100),
            ]),
            allowances: BTreeMap::new(),
        },
    };

    let msg = UnstakeMsg { amount: 151 };
    let (_, _) = unstake(mock_contract_context(JACK, block_time), state, msg);
}

#[test]
#[should_panic(expected = "Nothing to claim")]
fn claim_with_zero_rewards() {
    const DEPOSIT_TOKEN: u8 = 1;
    const MINTER: u8 = 9;
    const ALICE: u8 = 10;
    const BOB: u8 = 11;
    const JACK: u8 = 12;

    let block_time = 135;

    let state = MPC20StakingContractState {
        deposit_token: mock_address(DEPOSIT_TOKEN),
        distribution_amount: 1_000,
        distribution_epoch: 10,
        global_index: DecimalRatio::new(16666666666666666666666666667, 27),
        total_staked: 300,
        last_distributed: 135,
        stakers: BTreeMap::from([
            (
                mock_address(ALICE),
                Staker {
                    reward_index: DecimalRatio::new(16666666666666666666666666667, 27),
                    staked_amount: 100,
                    pending_reward: 1065,
                    last_compound: 0,
                },
            ),
            (
                mock_address(BOB),
                Staker {
                    reward_index: DecimalRatio::new(16666666666666666666666666667, 27),
                    staked_amount: 50,
                    pending_reward: 249,
                    last_compound: 0,
                },
            ),
            (
                mock_address(JACK),
                Staker {
                    reward_index: DecimalRatio::new(16666666666666666666666666667, 27),
                    staked_amount: 150,
                    pending_reward: 0,
                    last_compound: 144,
                },
            ),
        ]),
        compound_frequency: 100,
        mpc20_base_state: MPC20ContractState {
            info: TokenInfo {
                name: "Staking Token".to_string(),
                symbol: "STKN".to_string(),
                decimals: 18,
            },
            total_supply: 1_536,
            minter: Some(Minter {
                minter: mock_address(MINTER),
                capacity: None,
            }),
            balances: BTreeMap::from([
                (mock_address(ALICE), 601),
                (mock_address(BOB), 834),
                (mock_address(JACK), 1),
                (mock_address(DEPOSIT_TOKEN), 100),
            ]),
            allowances: BTreeMap::new(),
        },
    };

    let msg = ClaimMsg { amount: None };
    let (_, _) = claim(mock_contract_context(JACK, block_time), state, msg);
}

#[test]
#[should_panic(expected = "Cannot claim more then rewarded")]
fn claim_more_then_rewarded() {
    const DEPOSIT_TOKEN: u8 = 1;
    const MINTER: u8 = 9;
    const ALICE: u8 = 10;
    const BOB: u8 = 11;
    const JACK: u8 = 12;

    let block_time = 135;

    let state = MPC20StakingContractState {
        deposit_token: mock_address(DEPOSIT_TOKEN),
        distribution_amount: 1_000,
        distribution_epoch: 10,
        global_index: DecimalRatio::new(16666666666666666666666666667, 27),
        total_staked: 300,
        last_distributed: 135,
        stakers: BTreeMap::from([
            (
                mock_address(ALICE),
                Staker {
                    reward_index: DecimalRatio::new(16666666666666666666666666667, 27),
                    staked_amount: 100,
                    pending_reward: 1065,
                    last_compound: 0,
                },
            ),
            (
                mock_address(BOB),
                Staker {
                    reward_index: DecimalRatio::new(16666666666666666666666666667, 27),
                    staked_amount: 50,
                    pending_reward: 249,
                    last_compound: 0,
                },
            ),
            (
                mock_address(JACK),
                Staker {
                    reward_index: DecimalRatio::new(16666666666666666666666666667, 27),
                    staked_amount: 150,
                    pending_reward: 10,
                    last_compound: 144,
                },
            ),
        ]),
        compound_frequency: 100,
        mpc20_base_state: MPC20ContractState {
            info: TokenInfo {
                name: "Staking Token".to_string(),
                symbol: "STKN".to_string(),
                decimals: 18,
            },
            total_supply: 1_536,
            minter: Some(Minter {
                minter: mock_address(MINTER),
                capacity: None,
            }),
            balances: BTreeMap::from([
                (mock_address(ALICE), 601),
                (mock_address(BOB), 834),
                (mock_address(JACK), 1),
                (mock_address(DEPOSIT_TOKEN), 100),
            ]),
            allowances: BTreeMap::new(),
        },
    };

    let msg = ClaimMsg { amount: Some(11) };
    let (_, _) = claim(mock_contract_context(JACK, block_time), state, msg);
}

#[test]
#[should_panic(expected = "Compound only enabled when deposit token is reward token")]
fn compound_when_disabled() {
    const DEPOSIT_TOKEN: u8 = 2;
    const MINTER: u8 = 9;
    const ALICE: u8 = 10;
    const BOB: u8 = 11;
    const JACK: u8 = 12;

    let block_time = 135;

    let state = MPC20StakingContractState {
        deposit_token: mock_address(DEPOSIT_TOKEN),
        distribution_amount: 1_000,
        distribution_epoch: 10,
        global_index: DecimalRatio::new(16666666666666666666666666667, 27),
        total_staked: 300,
        last_distributed: 135,
        stakers: BTreeMap::from([
            (
                mock_address(ALICE),
                Staker {
                    reward_index: DecimalRatio::new(16666666666666666666666666667, 27),
                    staked_amount: 100,
                    pending_reward: 1065,
                    last_compound: 0,
                },
            ),
            (
                mock_address(BOB),
                Staker {
                    reward_index: DecimalRatio::new(16666666666666666666666666667, 27),
                    staked_amount: 50,
                    pending_reward: 249,
                    last_compound: 0,
                },
            ),
            (
                mock_address(JACK),
                Staker {
                    reward_index: DecimalRatio::new(16666666666666666666666666667, 27),
                    staked_amount: 150,
                    pending_reward: 10,
                    last_compound: 144,
                },
            ),
        ]),
        compound_frequency: 100,
        mpc20_base_state: MPC20ContractState {
            info: TokenInfo {
                name: "Staking Token".to_string(),
                symbol: "STKN".to_string(),
                decimals: 18,
            },
            total_supply: 1_536,
            minter: Some(Minter {
                minter: mock_address(MINTER),
                capacity: None,
            }),
            balances: BTreeMap::from([
                (mock_address(ALICE), 601),
                (mock_address(BOB), 834),
                (mock_address(JACK), 1),
                (mock_address(DEPOSIT_TOKEN), 100),
            ]),
            allowances: BTreeMap::new(),
        },
    };

    let msg = CompoundMsg { amount: None };
    let (_, _) = compound(mock_contract_context(BOB, block_time), state, msg);
}

#[test]
#[should_panic(expected = "Forbidden to compound to often")]
fn compound_to_often() {
    const DEPOSIT_TOKEN: u8 = 1;
    const MINTER: u8 = 9;
    const ALICE: u8 = 10;
    const BOB: u8 = 11;
    const JACK: u8 = 12;

    let block_time = 135;

    let state = MPC20StakingContractState {
        deposit_token: mock_address(DEPOSIT_TOKEN),
        distribution_amount: 1_000,
        distribution_epoch: 10,
        global_index: DecimalRatio::new(16666666666666666666666666667, 27),
        total_staked: 300,
        last_distributed: 135,
        stakers: BTreeMap::from([
            (
                mock_address(ALICE),
                Staker {
                    reward_index: DecimalRatio::new(16666666666666666666666666667, 27),
                    staked_amount: 100,
                    pending_reward: 1065,
                    last_compound: 0,
                },
            ),
            (
                mock_address(BOB),
                Staker {
                    reward_index: DecimalRatio::new(16666666666666666666666666667, 27),
                    staked_amount: 50,
                    pending_reward: 249,
                    last_compound: 36,
                },
            ),
            (
                mock_address(JACK),
                Staker {
                    reward_index: DecimalRatio::new(16666666666666666666666666667, 27),
                    staked_amount: 150,
                    pending_reward: 10,
                    last_compound: 144,
                },
            ),
        ]),
        compound_frequency: 100,
        mpc20_base_state: MPC20ContractState {
            info: TokenInfo {
                name: "Staking Token".to_string(),
                symbol: "STKN".to_string(),
                decimals: 18,
            },
            total_supply: 1_536,
            minter: Some(Minter {
                minter: mock_address(MINTER),
                capacity: None,
            }),
            balances: BTreeMap::from([
                (mock_address(ALICE), 601),
                (mock_address(BOB), 834),
                (mock_address(JACK), 1),
                (mock_address(DEPOSIT_TOKEN), 100),
            ]),
            allowances: BTreeMap::new(),
        },
    };

    let msg = CompoundMsg { amount: None };
    let (_, _) = compound(mock_contract_context(BOB, block_time), state, msg);
}

#[test]
#[should_panic(expected = "Cannot compound more then rewarded")]
fn compound_more_then_rewarded() {
    const DEPOSIT_TOKEN: u8 = 1;
    const MINTER: u8 = 9;
    const ALICE: u8 = 10;
    const BOB: u8 = 11;
    const JACK: u8 = 12;

    let block_time = 135;

    let state = MPC20StakingContractState {
        deposit_token: mock_address(DEPOSIT_TOKEN),
        distribution_amount: 1_000,
        distribution_epoch: 10,
        global_index: DecimalRatio::new(16666666666666666666666666667, 27),
        total_staked: 300,
        last_distributed: 135,
        stakers: BTreeMap::from([
            (
                mock_address(ALICE),
                Staker {
                    reward_index: DecimalRatio::new(16666666666666666666666666667, 27),
                    staked_amount: 100,
                    pending_reward: 1065,
                    last_compound: 0,
                },
            ),
            (
                mock_address(BOB),
                Staker {
                    reward_index: DecimalRatio::new(16666666666666666666666666667, 27),
                    staked_amount: 50,
                    pending_reward: 249,
                    last_compound: 0,
                },
            ),
            (
                mock_address(JACK),
                Staker {
                    reward_index: DecimalRatio::new(16666666666666666666666666667, 27),
                    staked_amount: 150,
                    pending_reward: 10,
                    last_compound: 144,
                },
            ),
        ]),
        compound_frequency: 100,
        mpc20_base_state: MPC20ContractState {
            info: TokenInfo {
                name: "Staking Token".to_string(),
                symbol: "STKN".to_string(),
                decimals: 18,
            },
            total_supply: 1_536,
            minter: Some(Minter {
                minter: mock_address(MINTER),
                capacity: None,
            }),
            balances: BTreeMap::from([
                (mock_address(ALICE), 601),
                (mock_address(BOB), 834),
                (mock_address(JACK), 1),
                (mock_address(DEPOSIT_TOKEN), 100),
            ]),
            allowances: BTreeMap::new(),
        },
    };

    let msg = CompoundMsg { amount: Some(250) };
    let (_, _) = compound(mock_contract_context(BOB, block_time), state, msg);
}
