use create_type_spec_derive::CreateTypeSpec;
use pbc_contract_common::address::Address;
use read_write_rpc_derive::ReadWriteRPC;

use crate::state::Vote;
use utils::events::NamedRPCEvent;

/// ## Description
/// This structure describes fields for mpc1-multisig initialize msg
#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct MultisigMember {
    /// multisig member address
    pub address: Address,
    /// member weight
    pub weight: u64,
}

/// ## Description
/// This structure describes fields for mpc1-multisig initialize msg
#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct InitMsg {
    /// multisig members
    pub members: Vec<MultisigMember>,
    /// required threshold
    pub threshold_weight: u64,
    /// voting phase period in UTC timestamp
    pub voting_phase_period: u64,
}

/// ## Description
/// This structure describes fields for mpc1-multisig create proposal msg
#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct CreateProposalMsg {
    /// proposal title
    pub title: String,
    /// proposal description
    pub description: String,
    /// optional voting period
    pub voting_phase_period: Option<u64>,
    /// proposal calls to execute
    pub calls: Vec<ProposalExecuteCallMsg>,
}

impl NamedRPCEvent for CreateProposalMsg {
    fn event_name(&self) -> String {
        "create_proposal".to_string()
    }
}

/// ## Description
/// This structure describes fields for mpc1-multisig proposal execute call msg
#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct ProposalExecuteCallMsg {
    /// contract to call
    pub contract: Address,
    /// method name to execute
    pub method_name: String,
    /// base64 encoded msg payload
    pub base64_encoded_payload: String,
}

/// ## Description
/// This structure describes fields for mpc1-multisig proposal vote msg
#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct ProposalVoteMsg {
    /// proposal id
    pub proposal_id: u64,
    /// vote type
    pub vote: Vote,
}

impl NamedRPCEvent for ProposalVoteMsg {
    fn event_name(&self) -> String {
        "vote".to_string()
    }
}

/// ## Description
/// This structure describes fields for mpc1-multisig proposal execute msg
#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct ProposalExecuteMsg {
    /// proposal id to execute
    pub proposal_id: u64,
}

impl NamedRPCEvent for ProposalExecuteMsg {
    fn event_name(&self) -> String {
        "execute_proposal".to_string()
    }
}

/// ## Description
/// This structure describes fields for mpc1-multisig proposal close msg
#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct ProposalCloseMsg {
    /// proposal id to close
    pub proposal_id: u64,
}

impl NamedRPCEvent for ProposalCloseMsg {
    fn event_name(&self) -> String {
        "close_proposal".to_string()
    }
}
