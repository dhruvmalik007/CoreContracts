use std::collections::BTreeMap;

use create_type_spec_derive::CreateTypeSpec;
use pbc_contract_common::address::Address;
use read_write_rpc_derive::ReadWriteRPC;
use read_write_state_derive::ReadWriteState;

#[derive(ReadWriteState, CreateTypeSpec, Clone, PartialEq, Debug)]
pub struct MPC1MultisigContractState {
    pub members: BTreeMap<Address, u64>,
    pub threshold_weight: u64,
    pub total_weight: u64,
    pub voting_phase_period: u64,
    pub proposals_count: u64,
    pub proposals: BTreeMap<u64, Proposal>,
}

impl MPC1MultisigContractState {
    pub fn save_proposal(&mut self, proposal: &Proposal) {
        self.proposals_count += 1;
        self.proposals
            .insert(self.proposals_count, proposal.clone());
    }
}

#[derive(ReadWriteRPC, ReadWriteState, CreateTypeSpec, Clone, PartialEq, Debug)]
pub struct Proposal {
    pub title: String,
    pub description: String,
    pub expires_at: u64,
    pub execute_calls: Vec<ProposalExecuteCall>,
    pub status: ProposalStatus,
    pub threshold_weight: u64,
    pub total_weight: u64,
    pub votes: SubmittedVotes,
    pub ballots: Vec<Ballot>,
}

impl Proposal {
    pub fn register_vote(&mut self, member: &Address, vote: Vote, weight: u64) {
        match vote {
            YES_VOTE => self.votes.yes += weight,
            NO_VOTE => self.votes.no += weight,
            _ => panic!("Unknown Vote type"),
        }

        self.ballots.push(Ballot {
            member: *member,
            vote,
            weight,
        });
    }

    pub fn update_status(&mut self, block_time: u64) {
        self.status = self.current_status(block_time)
    }

    pub fn mark_executed(&mut self) {
        self.status = EXECUTED_STATUS
    }

    pub fn mark_rejected(&mut self) {
        self.status = REJECTED_STATUS
    }

    pub fn not_voted(&self, member: &Address) -> bool {
        !self.ballots.iter().any(|b| b.member == *member)
    }

    pub fn not_expired(&self, block_time: u64) -> bool {
        block_time < self.expires_at
    }

    pub fn current_status(&self, block_time: u64) -> ProposalStatus {
        let mut status = self.status.clone();
        if status == VOTING_PHASE_STATUS {
            if self.is_passed() {
                status = ACCEPTED_STATUS;
            }

            if self.is_rejected() || !self.not_expired(block_time) {
                status = REJECTED_STATUS;
            }
        }

        status
    }

    pub fn is_passed(&self) -> bool {
        self.votes.yes >= self.threshold_weight
    }

    pub fn is_rejected(&self) -> bool {
        self.votes.no > (self.total_weight - self.threshold_weight)
    }
}

#[derive(ReadWriteRPC, ReadWriteState, CreateTypeSpec, Clone, PartialEq, Debug)]
pub struct ProposalExecuteCall {
    pub contract: Address,
    pub payload: Vec<u8>,
}

pub type ProposalStatus = u8;
pub const VOTING_PHASE_STATUS: ProposalStatus = 1;
pub const ACCEPTED_STATUS: ProposalStatus = 2;
pub const REJECTED_STATUS: ProposalStatus = 3;
pub const EXECUTED_STATUS: ProposalStatus = 4;

pub type Vote = u8;
pub const YES_VOTE: Vote = 1;
pub const NO_VOTE: Vote = 2;

#[derive(ReadWriteRPC, ReadWriteState, CreateTypeSpec, Clone, PartialEq, Debug)]
pub struct SubmittedVotes {
    pub yes: u64,
    pub no: u64,
}

impl SubmittedVotes {
    pub fn yes(weight: u64) -> Self {
        Self { yes: weight, no: 0 }
    }
}

#[derive(ReadWriteRPC, ReadWriteState, CreateTypeSpec, Clone, PartialEq, Debug)]
pub struct Ballot {
    pub member: Address,
    pub vote: Vote,
    pub weight: u64,
}
