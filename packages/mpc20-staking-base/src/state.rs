use rust_decimal::prelude::*;
use std::collections::BTreeMap;

use create_type_spec_derive::CreateTypeSpec;
use pbc_contract_common::address::Address;
use read_write_rpc_derive::ReadWriteRPC;
use read_write_state_derive::ReadWriteState;

use mpc20_base::state::MPC20ContractState;
use utils::decimal::DecimalRatio;

#[derive(ReadWriteState, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct MPC20StakingContractState {
    pub deposit_token: Address,
    pub distribution_amount: u128,
    pub distribution_epoch: u64,

    pub global_index: DecimalRatio,
    pub total_staked: u128,
    pub last_distributed: u64,

    pub stakers: BTreeMap<Address, Staker>,
    pub compound_frequency: u64,
    pub mpc20: MPC20ContractState,
}

impl MPC20StakingContractState {
    pub fn distribute_rewards(&mut self, block_time: u64) {
        if self.total_staked.is_zero() {
            self.last_distributed = block_time;
            return;
        }

        let passed_distributions = (block_time - self.last_distributed) / self.distribution_epoch;
        if passed_distributions.is_zero() {
            return;
        }

        let distributed_amount = self.distribution_amount * (passed_distributions as u128);
        self.global_index =
            self.global_index + DecimalRatio::from_ratio(distributed_amount, self.total_staked);
        self.last_distributed += self.distribution_epoch * passed_distributions;
    }

    pub fn increase_stake_amount(&mut self, address: &Address, staker: &mut Staker, amount: u128) {
        self.total_staked = self.total_staked.checked_add(amount).unwrap();
        staker.staked_amount = staker.staked_amount.checked_add(amount).unwrap();
        self.store_staker(address, staker);
    }

    pub fn decrease_stake_amount(&mut self, address: &Address, staker: &mut Staker, amount: u128) {
        self.total_staked = self.total_staked.checked_sub(amount).unwrap();
        staker.staked_amount = staker.staked_amount.checked_sub(amount).unwrap();
        self.store_staker(address, staker);
    }

    pub fn store_staker(&mut self, address: &Address, staker: &Staker) {
        self.stakers
            .entry(*address)
            .and_modify(|s| {
                s.reward_index = staker.reward_index;
                s.staked_amount = staker.staked_amount;
                s.pending_reward = staker.pending_reward;
                s.last_compound = staker.last_compound;
            })
            .or_insert_with(|| staker.clone());
    }

    pub fn get_staker(&self, address: &Address) -> Staker {
        match self.stakers.get(address) {
            Some(s) => s.clone(),
            None => Staker {
                reward_index: DecimalRatio::default(),
                staked_amount: 0,
                pending_reward: 0,
                last_compound: 0,
            },
        }
    }
}

#[derive(ReadWriteRPC, ReadWriteState, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct Staker {
    pub reward_index: DecimalRatio,
    pub staked_amount: u128,
    pub pending_reward: u128,
    pub last_compound: u64,
}

impl Staker {
    pub fn compute_reward(&mut self, global_index: DecimalRatio) {
        let staked_amount = DecimalRatio::new(self.staked_amount, 0);
        let pending_reward = (staked_amount * global_index) - (staked_amount * self.reward_index);

        self.reward_index = global_index;
        self.pending_reward = self
            .pending_reward
            .checked_add(pending_reward.to_u128())
            .unwrap();
    }
}
