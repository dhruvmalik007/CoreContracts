use contract_version_base::state::ContractVersionBase;
use mpc20_base::state::MPC20ContractState;

#[state]
#[derive(PartialEq, Eq, Debug)]
pub struct ContractState {
    pub mpc20: MPC20ContractState,
    pub version: ContractVersionBase,
}
