use token_sale_base::state::TokenSaleContractState;

#[state]
#[derive(PartialEq, Debug)]
pub struct ContractState {
    pub token_sale: TokenSaleContractState,
}

