use create_type_spec_derive::CreateTypeSpec;
use pbc_contract_common::address::Address;
use read_write_rpc_derive::ReadWriteRPC;

use crate::state::Vote;
use utils::events::NamedRPCEvent;

#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct MultisigMember {
    pub address: Address,
    pub weight: u64,
}

#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct InitMsg {
    pub members: Vec<MultisigMember>,
    pub threshold_weight: u64,
    pub voting_phase_period: u64,
}

#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct CreateProposalMsg {
    pub title: String,
    pub description: String,
    pub voting_phase_period: Option<u64>,
    pub calls: Vec<ProposalExecuteCallMsg>,
}

impl NamedRPCEvent for CreateProposalMsg {
    fn event_name(&self) -> String {
        "create_proposal".to_string()
    }
}

#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct ProposalExecuteCallMsg {
    pub contract: Address,
    pub method_name: String,
    pub base64_encoded_payload: String,
}

#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct ProposalVoteMsg {
    pub proposal_id: u64,
    pub vote: Vote,
}

impl NamedRPCEvent for ProposalVoteMsg {
    fn event_name(&self) -> String {
        "vote".to_string()
    }
}

#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct ProposalExecuteMsg {
    pub proposal_id: u64,
}

impl NamedRPCEvent for ProposalExecuteMsg {
    fn event_name(&self) -> String {
        "execute_proposal".to_string()
    }
}

#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct ProposalCloseMsg {
    pub proposal_id: u64,
}

impl NamedRPCEvent for ProposalCloseMsg {
    fn event_name(&self) -> String {
        "close_proposal".to_string()
    }
}
