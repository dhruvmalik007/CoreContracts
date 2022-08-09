use mpc721_base::state::MPC721ContractState;

#[state]
#[derive(PartialEq, Debug)]
pub struct ContractState {
    pub mpc721: MPC721ContractState,
}
