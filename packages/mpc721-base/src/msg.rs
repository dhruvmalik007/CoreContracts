use create_type_spec_derive::CreateTypeSpec;
use pbc_contract_common::address::{Address, Shortname};
use read_write_rpc_derive::ReadWriteRPC;

use utils::events::IntoShortnameRPCEvent;

/// ## Description
/// This structure describes fields for mpc721 initialize msg
#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct InitMsg {
    /// optional owner address
    pub owner: Option<Address>,
    /// token name
    pub name: String,
    /// token symbol
    pub symbol: String,
    /// optional base uri
    pub base_uri: Option<String>,
    /// token minter address
    pub minter: Address,
}

/// ## Description
/// This structure describes fields for mpc721 transfer msg
#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct TransferMsg {
    /// receiver address
    pub to: Address,
    /// token id
    pub token_id: u128,
}

impl IntoShortnameRPCEvent for TransferMsg {
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
            .argument(self.to)
            .argument(self.token_id)
            .done();
    }
}

/// ## Description
/// This structure describes fields for mpc721 transfer from msg
#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct TransferFromMsg {
    /// owner address
    pub from: Address,
    /// receiver address
    pub to: Address,
    /// token id
    pub token_id: u128,
}

impl IntoShortnameRPCEvent for TransferFromMsg {
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
            .argument(self.token_id)
            .done();
    }
}

/// ## Description
/// This structure describes fields for mpc721 approve msg
#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct ApproveMsg {
    /// operator address to approve
    pub spender: Address,
    /// token id
    pub token_id: u128,
}

impl IntoShortnameRPCEvent for ApproveMsg {
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
            .argument(self.spender)
            .argument(self.token_id)
            .done();
    }
}

/// ## Description
/// This structure describes fields for mpc721 set base uri msg
#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct SetBaseUriMsg {
    /// new base uri
    pub new_base_uri: String,
}

impl IntoShortnameRPCEvent for SetBaseUriMsg {
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
            .argument(self.new_base_uri.clone())
            .done();
    }
}

/// ## Description
/// This structure describes fields for mpc721 mint msg
#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct MintMsg {
    /// newly minted token id
    pub token_id: u128,
    /// receiver address
    pub to: Address,
    /// optional token uri
    pub token_uri: Option<String>,
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
            .argument(self.token_id)
            .argument(self.to)
            .argument(self.token_uri.clone())
            .done();
    }
}

/// ## Description
/// This structure describes fields for mpc721 approve for all msg
#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct ApproveForAllMsg {
    /// operator address to approve
    pub operator: Address,
}

impl IntoShortnameRPCEvent for ApproveForAllMsg {
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
            .argument(self.operator)
            .done();
    }
}
/// ## Description
/// This structure describes fields for mpc721 revoke msg
#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct RevokeMsg {
    /// operator address to revoke
    pub spender: Address,
    /// token id
    pub token_id: u128,
}

impl IntoShortnameRPCEvent for RevokeMsg {
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
            .argument(self.spender)
            .argument(self.token_id)
            .done();
    }
}

/// ## Description
/// This structure describes fields for mpc721 revoke for all msg
#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct RevokeForAllMsg {
    /// operator address to revoke
    pub operator: Address,
}

impl IntoShortnameRPCEvent for RevokeForAllMsg {
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
            .argument(self.operator)
            .done();
    }
}

/// ## Description
/// This structure describes fields for mpc721 burn msg
#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct BurnMsg {
    /// token id to burn
    pub token_id: u128,
}

impl IntoShortnameRPCEvent for BurnMsg {
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
            .argument(self.token_id)
            .done();
    }
}

/// ## Description
/// This structure describes fields for mpc721 check owner msg
#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct CheckOwnerMsg {
    /// receiver address
    pub owner: Address,
    /// token id
    pub token_id: u128,
}

impl IntoShortnameRPCEvent for CheckOwnerMsg {
    fn action_shortname(&self) -> u32 {
        0x18
    }

    fn as_interaction(
        &self,
        builder: &mut pbc_contract_common::events::EventGroupBuilder,
        dest: &Address,
    ) {
        builder
            .call(*dest, Shortname::from_u32(self.action_shortname()))
            .argument(self.owner)
            .argument(self.token_id)
            .done();
    }
}
