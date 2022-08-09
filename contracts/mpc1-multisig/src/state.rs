use mpc1_multisig_base::state::MPC1MultisigContractState;

#[state]
#[derive(PartialEq, Debug)]
pub struct ContractState {
    pub mpc1_multisig: MPC1MultisigContractState,
}
