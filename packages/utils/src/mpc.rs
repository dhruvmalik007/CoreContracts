use pbc_contract_common::{
    address::{Address, AddressType, Shortname},
    events::EventGroupBuilder,
};

// 01a4082d9d560749ecd0ffa1dcaaaee2c2cb25d881
const MAINNET_MPC_TOKEN: Address = Address {
    address_type: AddressType::SystemContract,
    identifier: [
        0xa4, 0x08, 0x2d, 0x9d, 0x56, 0x07, 0x49, 0xec, 0xd0, 0xff, 0xa1, 0xdc, 0xaa, 0xae, 0xe2,
        0xc2, 0xcb, 0x25, 0xd8, 0x81,
    ],
};

pub fn native_mpc_transfer(
    event_group: &mut EventGroupBuilder,
    to: Address,
    amount: i64,
    from_origin: bool,
) {
    let mut interaction = event_group.call(MAINNET_MPC_TOKEN, Shortname::from_u32(0x03));
    if from_origin {
        interaction = interaction.from_original_sender();
    }

    interaction.argument(to).argument(amount).done();
}
