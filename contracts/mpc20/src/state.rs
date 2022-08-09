use mpc20_base::state::MPC20ContractState;
#[state]
#[derive(PartialEq, Debug)]
pub struct ContractState {
    pub mpc20: MPC20ContractState,
}
