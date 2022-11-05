use create_type_spec_derive::CreateTypeSpec;
use pbc_contract_common::address::{Address, Shortname};
use read_write_rpc_derive::ReadWriteRPC;

use utils::events::IntoShortnameRPCEvent;

/// ## Description
/// This structure describes fields for mpc1155 initialize msg
#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct InitMsg {
    /// optional owner address
    pub owner: Option<Address>,
    /// base uri
    pub uri: String,
    /// minter address
    pub minter: Address,
}

/// ## Description
/// This structure describes fields for mpc1155 transfer from msg
#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct TransferFromMsg {
    /// owner address
    pub from: Address,
    /// receiver address
    pub to: Address,
    /// token info for transfer
    pub token_info: TokenTransferInfoMsg,
}

impl IntoShortnameRPCEvent for TransferFromMsg {
    fn action_shortname(&self) -> u32 {
        0x01
    }

    fn as_interaction(
        &self,
        builder: &mut pbc_contract_common::events::EventGroupBuilder,
        dest: &Address,
    ) {
        builder
            .call(*dest, Shortname::from_u32(self.action_shortname()))
            .argument(self.from)
            .argument(self.to)
            .argument(self.token_info.clone())
            .done();
    }
}

/// ## Description
/// This structure describes fields for mpc1155 batch transfer from msg
#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct BatchTransferFromMsg {
    /// owner address
    pub from: Address,
    /// receiver address
    pub to: Address,
    /// list of token infos for transfer
    pub token_infos: Vec<TokenTransferInfoMsg>,
}

impl IntoShortnameRPCEvent for BatchTransferFromMsg {
    fn action_shortname(&self) -> u32 {
        0x03
    }

    fn as_interaction(
        &self,
        builder: &mut pbc_contract_common::events::EventGroupBuilder,
        dest: &Address,
    ) {
        builder
            .call(*dest, Shortname::from_u32(self.action_shortname()))
            .argument(self.from)
            .argument(self.to)
            .argument(self.token_infos.clone())
            .done();
    }
}

/// ## Description
/// This structure describes fields for mpc1155 approve for all msg
#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct ApproveForAllMsg {
    /// operator address to approve
    pub operator: Address,
}

impl IntoShortnameRPCEvent for ApproveForAllMsg {
    fn action_shortname(&self) -> u32 {
        0x05
    }

    fn as_interaction(
        &self,
        builder: &mut pbc_contract_common::events::EventGroupBuilder,
        dest: &Address,
    ) {
        builder
            .call(*dest, Shortname::from_u32(self.action_shortname()))
            .argument(self.operator)
            .done();
    }
}

/// ## Description
/// This structure describes fields for mpc1155 set uri msg
#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct SetUriMsg {
    /// new base uri
    pub new_uri: String,
}

impl IntoShortnameRPCEvent for SetUriMsg {
    fn action_shortname(&self) -> u32 {
        0x07
    }

    fn as_interaction(
        &self,
        builder: &mut pbc_contract_common::events::EventGroupBuilder,
        dest: &Address,
    ) {
        builder
            .call(*dest, Shortname::from_u32(self.action_shortname()))
            .argument(self.new_uri.clone())
            .done();
    }
}

/// ## Description
/// This structure describes fields for mpc1155 mint msg
#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct TokenMintInfoMsg {
    /// token id
    pub token_id: u128,
    /// amount of token to mint
    pub amount: u128,
    /// optional token uri
    pub token_uri: Option<String>,
}

/// ## Description
/// This structure describes fields for mpc1155 mint msg
#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct MintMsg {
    pub to: Address,
    pub token_info: TokenMintInfoMsg,
}

impl IntoShortnameRPCEvent for MintMsg {
    fn action_shortname(&self) -> u32 {
        0x09
    }

    fn as_interaction(
        &self,
        builder: &mut pbc_contract_common::events::EventGroupBuilder,
        dest: &Address,
    ) {
        builder
            .call(*dest, Shortname::from_u32(self.action_shortname()))
            .argument(self.to)
            .argument(self.token_info.clone())
            .done();
    }
}

/// ## Description
/// This structure describes fields for mpc1155 batch mint msg
#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct BatchMintMsg {
    /// receiver address
    pub to: Address,
    /// list of tokens to mint
    pub token_infos: Vec<TokenMintInfoMsg>,
}

impl IntoShortnameRPCEvent for BatchMintMsg {
    fn action_shortname(&self) -> u32 {
        0x11
    }

    fn as_interaction(
        &self,
        builder: &mut pbc_contract_common::events::EventGroupBuilder,
        dest: &Address,
    ) {
        builder
            .call(*dest, Shortname::from_u32(self.action_shortname()))
            .argument(self.to)
            .argument(self.token_infos.clone())
            .done();
    }
}

/// ## Description
/// This structure describes fields for mpc1155 transfer msg
#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct TokenTransferInfoMsg {
    /// token id
    pub token_id: u128,
    /// amount of tokens to transfer
    pub amount: u128,
}

/// ## Description
/// This structure describes fields for mpc1155 burn msg
#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct BurnMsg {
    /// owner address
    pub from: Address,
    /// token info for burn
    pub token_info: TokenTransferInfoMsg,
}

impl IntoShortnameRPCEvent for BurnMsg {
    fn action_shortname(&self) -> u32 {
        0x13
    }

    fn as_interaction(
        &self,
        builder: &mut pbc_contract_common::events::EventGroupBuilder,
        dest: &Address,
    ) {
        builder
            .call(*dest, Shortname::from_u32(self.action_shortname()))
            .argument(self.from)
            .argument(self.token_info.clone())
            .done();
    }
}

/// ## Description
/// This structure describes fields for mpc1155 batch burn msg
#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct BatchBurnMsg {
    /// owner address
    pub from: Address,
    /// list of token infos for burn
    pub token_infos: Vec<TokenTransferInfoMsg>,
}

impl IntoShortnameRPCEvent for BatchBurnMsg {
    fn action_shortname(&self) -> u32 {
        0x15
    }

    fn as_interaction(
        &self,
        builder: &mut pbc_contract_common::events::EventGroupBuilder,
        dest: &Address,
    ) {
        builder
            .call(*dest, Shortname::from_u32(self.action_shortname()))
            .argument(self.from)
            .argument(self.token_infos.clone())
            .done();
    }
}

/// ## Description
/// This structure describes fields for mpc1155 revoke for all msg
#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct RevokeForAllMsg {
    /// operator address to revoke
    pub operator: Address,
}

impl IntoShortnameRPCEvent for RevokeForAllMsg {
    fn action_shortname(&self) -> u32 {
        0x17
    }

    fn as_interaction(
        &self,
        builder: &mut pbc_contract_common::events::EventGroupBuilder,
        dest: &Address,
    ) {
        builder
            .call(*dest, Shortname::from_u32(self.action_shortname()))
            .argument(self.operator)
            .done();
    }
}
