use contract_version_base::state::ContractVersionBase;
use mpc20_staking_base::state::MPC20StakingContractState;

#[state]
#[derive(PartialEq, Eq, Debug)]
pub struct ContractState {
    pub mpc20_staking: MPC20StakingContractState,
    pub version: ContractVersionBase,
}
