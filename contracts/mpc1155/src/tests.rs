use mpc1155_base::msg::{
    ApproveForAllMsg, BatchBurnMsg, BatchMintMsg, BatchTransferFromMsg, BurnMsg, MintMsg,
    RevokeForAllMsg, SetUriMsg, TokenMintInfoMsg, TokenTransferInfoMsg, TransferFromMsg,
};
use pbc_contract_common::{
    address::{Address, AddressType, Shortname},
    events::EventGroup,
};
use utils::events::IntoShortnameRPCEvent;

fn mock_address(le: u8) -> Address {
    Address {
        address_type: AddressType::Account,
        identifier: [
            le, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
            0u8, 0u8, 0u8,
        ],
    }
}

const TRANSFER_FROM: u32 = 0x01;
const BATCH_TRANSFER_FROM: u32 = 0x03;
const APPROVE_FOR_ALL: u32 = 0x05;
const SET_URI: u32 = 0x07;
const MINT: u32 = 0x09;
const BATCH_MINT: u32 = 0x11;
const BURN: u32 = 0x13;
const BATCH_BURN: u32 = 0x15;
const REVOKE_FOR_ALL: u32 = 0x17;

#[test]
fn proper_transfer_from_action_call() {
    let dest = mock_address(30u8);

    let msg = TransferFromMsg {
        from: mock_address(1u8),
        to: mock_address(2u8),
        token_info: TokenTransferInfoMsg {
            token_id: 1,
            amount: 100,
        },
    };

    let mut event_group = EventGroup::builder();
    msg.as_interaction(&mut event_group, &dest);

    let mut test_event_group = EventGroup::builder();
    test_event_group
        .call(dest.clone(), Shortname::from_u32(TRANSFER_FROM))
        .argument(mock_address(1u8))
        .argument(mock_address(2u8))
        .argument(TokenTransferInfoMsg {
            token_id: 1,
            amount: 100,
        })
        .done();

    assert_eq!(event_group.build(), test_event_group.build());
}

#[test]
fn proper_batch_transfer_from_action_call() {
    let dest = mock_address(30u8);

    let msg = BatchTransferFromMsg {
        from: mock_address(1u8),
        to: mock_address(2u8),
        token_infos: vec![TokenTransferInfoMsg {
            token_id: 1,
            amount: 100,
        }],
    };

    let mut event_group = EventGroup::builder();
    msg.as_interaction(&mut event_group, &dest);

    let mut test_event_group = EventGroup::builder();
    test_event_group
        .call(dest.clone(), Shortname::from_u32(BATCH_TRANSFER_FROM))
        .argument(mock_address(1u8))
        .argument(mock_address(2u8))
        .argument(vec![TokenTransferInfoMsg {
            token_id: 1,
            amount: 100,
        }])
        .done();

    assert_eq!(event_group.build(), test_event_group.build());
}

#[test]
fn proper_approve_for_all_action_call() {
    let dest = mock_address(30u8);

    let msg = ApproveForAllMsg {
        operator: mock_address(1u8),
    };

    let mut event_group = EventGroup::builder();
    msg.as_interaction(&mut event_group, &dest);

    let mut test_event_group = EventGroup::builder();
    test_event_group
        .call(dest.clone(), Shortname::from_u32(APPROVE_FOR_ALL))
        .argument(mock_address(1u8))
        .done();

    assert_eq!(event_group.build(), test_event_group.build());
}

#[test]
fn proper_set_uri_action_call() {
    let dest = mock_address(30u8);

    let msg = SetUriMsg {
        new_uri: "new".to_string(),
    };

    let mut event_group = EventGroup::builder();
    msg.as_interaction(&mut event_group, &dest);

    let mut test_event_group = EventGroup::builder();
    test_event_group
        .call(dest.clone(), Shortname::from_u32(SET_URI))
        .argument("new".to_string())
        .done();

    assert_eq!(event_group.build(), test_event_group.build());
}

#[test]
fn proper_mint_action_call() {
    let dest = mock_address(30u8);

    let msg = MintMsg {
        to: mock_address(1u8),
        token_info: TokenMintInfoMsg {
            token_id: 1,
            amount: 100,
            token_uri: Some("uri".to_string()),
        },
    };

    let mut event_group = EventGroup::builder();
    msg.as_interaction(&mut event_group, &dest);

    let mut test_event_group = EventGroup::builder();
    test_event_group
        .call(dest.clone(), Shortname::from_u32(MINT))
        .argument(mock_address(1u8))
        .argument(TokenMintInfoMsg {
            token_id: 1,
            amount: 100,
            token_uri: Some("uri".to_string()),
        })
        .done();

    assert_eq!(event_group.build(), test_event_group.build());
}

#[test]
fn proper_batch_mint_action_call() {
    let dest = mock_address(30u8);

    let msg = BatchMintMsg {
        to: mock_address(1u8),
        token_infos: vec![TokenMintInfoMsg {
            token_id: 1,
            amount: 100,
            token_uri: Some("uri".to_string()),
        }],
    };

    let mut event_group = EventGroup::builder();
    msg.as_interaction(&mut event_group, &dest);

    let mut test_event_group = EventGroup::builder();
    test_event_group
        .call(dest.clone(), Shortname::from_u32(BATCH_MINT))
        .argument(mock_address(1u8))
        .argument(vec![TokenMintInfoMsg {
            token_id: 1,
            amount: 100,
            token_uri: Some("uri".to_string()),
        }])
        .done();

    assert_eq!(event_group.build(), test_event_group.build());
}

#[test]
fn proper_burn_action_call() {
    let dest = mock_address(30u8);

    let msg = BurnMsg {
        from: mock_address(1u8),
        token_info: TokenTransferInfoMsg {
            token_id: 1,
            amount: 100,
        },
    };

    let mut event_group = EventGroup::builder();
    msg.as_interaction(&mut event_group, &dest);

    let mut test_event_group = EventGroup::builder();
    test_event_group
        .call(dest.clone(), Shortname::from_u32(BURN))
        .argument(mock_address(1u8))
        .argument(TokenTransferInfoMsg {
            token_id: 1,
            amount: 100,
        })
        .done();

    assert_eq!(event_group.build(), test_event_group.build());
}

#[test]
fn proper_batch_burn_action_call() {
    let dest = mock_address(30u8);

    let msg = BatchBurnMsg {
        from: mock_address(1u8),
        token_infos: vec![TokenTransferInfoMsg {
            token_id: 1,
            amount: 100,
        }],
    };

    let mut event_group = EventGroup::builder();
    msg.as_interaction(&mut event_group, &dest);

    let mut test_event_group = EventGroup::builder();
    test_event_group
        .call(dest.clone(), Shortname::from_u32(BATCH_BURN))
        .argument(mock_address(1u8))
        .argument(vec![TokenTransferInfoMsg {
            token_id: 1,
            amount: 100,
        }])
        .done();

    assert_eq!(event_group.build(), test_event_group.build());
}

#[test]
fn proper_revoke_for_all_action_call() {
    let dest = mock_address(30u8);

    let msg = RevokeForAllMsg {
        operator: mock_address(1u8),
    };

    let mut event_group = EventGroup::builder();
    msg.as_interaction(&mut event_group, &dest);

    let mut test_event_group = EventGroup::builder();
    test_event_group
        .call(dest.clone(), Shortname::from_u32(REVOKE_FOR_ALL))
        .argument(mock_address(1u8))
        .done();

    assert_eq!(event_group.build(), test_event_group.build());
}
