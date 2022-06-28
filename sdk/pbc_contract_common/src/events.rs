//! Definitions for RPC calls between contracts
//!
//! # Motivation
//!
//! Partisia Blockchain's contract interaction model sandboxes each contract, and allows RPC
//! calls as the primary form of interaction. As each transaction is entirely isolated, RPCs can
//! only occur "between" action calls.
//!
//! Abstractly, for example:
//!
//! - X calls Alice in transaction 1: Alice determine it needs some information from Bob, and exits
//!   while telling the blockchain: "Call Bob for me, I want a reply, and let me pay for the reply"
//! - Alice calls Bob in transaction 2: Bob performs it's computation, and exists with "Call Alice for
//!   me, she said she'd pay for this reply".
//! - Bob calls Alice in transaction 3: Alice got the necessary information to perform her own
//!   computation...
//!
//! # Implementation
//!
//! To accommodate the model, the SDK requires each `action` annotated function to return
//! a (possibly empty) `Vec` of `EventGroup`s, which represents the "Call X for me" information.
//!
//! Each `EventGroup` consists of one or more interactions (representing "Call X for me",) with the
//! possiblity of callbacks (representing "I want a reply".) All interactions in an `EventGroup`
//! shares gas costs uniformly.

use read_write_rpc_derive::ReadWriteRPC;

use crate::address::Address;

/// An interaction is what is sent from an event.
/// It consists of:
///
/// - `dest` - the address of the receiver
/// - `payload` - the raw payload to send to the receiver
/// - `from_contract` - whether to send the interaction from the contract or from the sender of the original transaction
/// - `cost` - the max cost of the interaction.
///
/// Serialized with the RPC format.
#[derive(ReadWriteRPC, Eq, PartialEq, Debug)]
pub struct Interaction {
    dest: Address,
    payload: Vec<u8>,
    from_original_sender: bool,
    cost: Option<u64>,
}

/// A callback is a simple interaction that is sent *after* all sent events have been processed
/// by a node on the chain.
///
/// - `rpc` - the raw RPC you want to receive
/// - `cost` - the max cost of the callback. If set to `None` the max cost is automatically set from the remaining gas.
///
/// Serialized with the RPC format.
#[derive(ReadWriteRPC, Eq, PartialEq, Debug)]
pub struct Callback {
    rpc: Vec<u8>,
    cost: Option<u64>,
}

/// The event group is a struct holding a list of events to send to other contracts and
/// an optional callback RPC.
///
/// See docs for `Interaction`.
///
/// Serialized with the RPC format.
#[derive(ReadWriteRPC, Eq, PartialEq, Debug)]
pub struct EventGroup {
    callback: Option<Vec<u8>>,
    cost: Option<u64>,
    events: Vec<Interaction>,
}

impl Default for EventGroup {
    fn default() -> Self {
        EventGroup::new()
    }
}

impl EventGroup {
    /// Construct a new empty event group.
    pub fn new() -> EventGroup {
        EventGroup {
            events: Vec::new(),
            callback: None,
            cost: None,
        }
    }

    /// Send an interaction with this current contract as sender.
    ///
    /// Params:
    /// - `dest`: Address of contract to call.
    /// - `payload`: Payload for the recipient contract.
    /// - `cost`: How much gas to dedicate to the callback. If `None` the cost is automatically set
    ///   from the remaining gas.
    pub fn send_from_contract(&mut self, dest: &Address, payload: Vec<u8>, cost: Option<u64>) {
        self.events.push(Interaction {
            dest: *dest,
            payload,
            from_original_sender: false,
            cost,
        })
    }

    /// Send an interaction with the original sender as sender.
    ///
    /// Params:
    /// - `dest`: Address of contract to call.
    /// - `payload`: Payload for the recipient contract.
    /// - `cost`: How much gas to dedicate to the callback. If `None` the cost is automatically set
    ///   from the remaining gas.
    pub fn send_from_original_sender(
        &mut self,
        dest: &Address,
        payload: Vec<u8>,
        cost: Option<u64>,
    ) {
        self.events.push(Interaction {
            dest: *dest,
            payload,
            from_original_sender: true,
            cost,
        })
    }

    /// Register a callback on this event group.
    ///
    /// Params:
    /// - `rpc`: Data to accompany the callback once it occurs.
    /// - `cost`: How much gas to dedicate to the callback. If `None` the cost is automatically set
    ///   from the remaining gas.
    pub fn register_callback(&mut self, rpc: Vec<u8>, cost: Option<u64>) {
        self.callback = Some(rpc);
        self.cost = cost;
    }
}

#[cfg(test)]
#[path = "../unit_tests/event_serialization.rs"]
mod event_serialization;
