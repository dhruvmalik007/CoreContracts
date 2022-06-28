//! Testing module with example data.
//!
//! Publicly available, as it's depended upon from other modules.
#![allow(dead_code)]

use crate::address::{Address, AddressType};
use crate::context::{CallbackContext, ContractContext, ExecutionResult};
#[cfg(any(feature = "zk", doc))]
use crate::zk;
use crate::Hash;

/// Example address
pub const EXAMPLE_ADDRESS_1: Address = Address {
    address_type: AddressType::PublicContract,
    identifier: [
        2, 32, 3, 2, 2, 3, 2, 3, 2, 32, 32, 3, 23, 2, 3, 23, 2, 3, 23, 2,
    ],
};

/// Example address
pub const EXAMPLE_ADDRESS_2: Address = Address {
    address_type: AddressType::Account,
    identifier: [
        29, 3, 3, 2, 2, 3, 2, 3, 2, 32, 32, 3, 23, 2, 3, 23, 2, 3, 23, 2,
    ],
};

/// Example Hash
pub const EXAMPLE_HASH_1: Hash = [
    0, 1, 23, 213, 124, 23, 3, 1, 23, 12, 31, 23, 123, 24, 3, 2, 2, 3, 2, 3, 2, 32, 32, 3, 23, 2,
    3, 23, 2, 3, 23, 2,
];

/// Example Hash
pub const EXAMPLE_HASH_2: Hash = [
    124, 25, 3, 1, 23, 12, 31, 23, 123, 26, 13, 3, 123, 32, 3, 2, 2, 3, 2, 3, 2, 32, 32, 3, 23, 2,
    3, 23, 2, 3, 23, 2,
];

/// Example contract context
pub const EXAMPLE_CONTEXT: ContractContext = ContractContext {
    contract_address: EXAMPLE_ADDRESS_1,
    sender: EXAMPLE_ADDRESS_2,
    block_time: 53,
    block_production_time: 53,
    current_transaction: EXAMPLE_HASH_1,
    original_transaction: EXAMPLE_HASH_2,
};

/// Generator of example callback contexts
pub fn example_callback_context() -> CallbackContext {
    CallbackContext {
        success: true,
        results: vec![
            ExecutionResult { succeeded: true },
            ExecutionResult { succeeded: true },
            ExecutionResult { succeeded: true },
        ],
    }
}

/// Defines the Metadata type used for [`zk::ZkClosed`] example instances
pub type ExampleZkMetadata = u32;

/// Example secret variable id
#[cfg(any(feature = "zk", doc))]
pub const SECRET_VAR_ID_31: zk::SecretVarId = zk::SecretVarId::new(31);

/// Example secret variable id
#[cfg(any(feature = "zk", doc))]
pub const SECRET_VAR_ID_30: zk::SecretVarId = zk::SecretVarId::new(30);

/// Example secret variable id
#[cfg(any(feature = "zk", doc))]
pub const SECRET_VAR_ID_4: zk::SecretVarId = zk::SecretVarId::new(4);

/// Example ZkClosed 1
///
/// Metadata is explicitly NOT palindromic wrt. endianess.
#[cfg(any(feature = "zk", doc))]
pub const ZK_CLOSED_1: zk::ZkClosed<ExampleZkMetadata> = zk::ZkClosed {
    variable_id: SECRET_VAR_ID_31,
    owner: EXAMPLE_ADDRESS_1,
    is_sealed: false,
    metadata: 0xFF,
    data: None,
};

/// Example ZkClosed 2
///
/// Metadata is explicitly NOT palindromic wrt. endianess.
#[cfg(any(feature = "zk", doc))]
pub const ZK_CLOSED_2: zk::ZkClosed<ExampleZkMetadata> = zk::ZkClosed {
    variable_id: SECRET_VAR_ID_30,
    owner: EXAMPLE_ADDRESS_2,
    is_sealed: false,
    metadata: 0xFF00,
    data: None,
};

/// Generator of open example ZkClosed
///
/// Metadata and data is explicitly NOT palindromic wrt. endianess.
#[cfg(any(feature = "zk", doc))]
pub fn zk_closed_open() -> zk::ZkClosed<ExampleZkMetadata> {
    zk::ZkClosed {
        variable_id: SECRET_VAR_ID_4,
        owner: EXAMPLE_ADDRESS_1,
        is_sealed: false,
        metadata: 0xDEADBEEF,
        data: Some(vec![1, 2, 3]),
    }
}

/// Generator of ZkInputDef examples
///
/// Metadata is explicitly NOT palindromic wrt. endianess.
#[cfg(any(feature = "zk", doc))]
pub fn zk_input_def(seed: u32) -> zk::ZkInputDef<ExampleZkMetadata> {
    assert_ne!(seed, u32::from_be(seed));
    zk::ZkInputDef {
        seal: seed % 2 == 0,
        expected_bit_lengths: (1..(seed % 10 + 2)).collect(),
        metadata: seed,
    }
}

/// Generator of example callback contexts
#[cfg(any(feature = "zk", doc))]
pub fn example_zk_state() -> zk::ZkState<ExampleZkMetadata> {
    zk::ZkState {
        calculation_state: zk::CalculationStatus::Waiting,
        pending_inputs: vec![ZK_CLOSED_1],
        secret_variables: vec![ZK_CLOSED_2, zk_closed_open()],
    }
}
