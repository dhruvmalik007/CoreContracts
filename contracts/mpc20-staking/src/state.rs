use mpc20_staking_base::state::MPC20StakingContractState;

#[state]
#[derive(PartialEq, Debug)]
pub struct ContractState {
    pub mpc20_staking: MPC20StakingContractState,
}