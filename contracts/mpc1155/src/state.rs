use contract_version_base::state::ContractVersionBase;
use mpc1155_base::state::MPC1155ContractState;

#[state]
#[derive(PartialEq, Eq, Debug)]
pub struct ContractState {
    pub mpc1155: MPC1155ContractState,
    pub version: ContractVersionBase,
}
