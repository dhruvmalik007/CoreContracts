use pbc_contract_common::{
    address::{Address, AddressType, Shortname},
    context::ContractContext,
    events::EventGroupBuilder,
};
use pbc_traits::ReadWriteRPC;

pub const CONTRACT_DEPLOYER: Address = Address {
    address_type: AddressType::SystemContract,
    identifier: [
        0x97, 0xa0, 0xe2, 0x38, 0xe9, 0x24, 0x02, 0x5b, 0xad, 0x14, 0x4a, 0xa0, 0xc4, 0x91, 0x3e,
        0x46, 0x30, 0x8f, 0x9a, 0x4d,
    ],
};

pub fn add_contract_deploy_event_with_msg<T>(
    ctx: &ContractContext,
    event_group: &mut EventGroupBuilder,
    wasm: &[u8],
    abi: &[u8],
    init_msg: &T,
) -> Address
where
    T: ReadWriteRPC,
{
    let mut raw_init_msg: Vec<u8> = vec![];
    init_msg.rpc_write_to(&mut raw_init_msg).unwrap();

    add_contract_deploy_event(ctx, event_group, wasm, abi, &raw_init_msg)
}

pub fn add_contract_deploy_event(
    ctx: &ContractContext,
    event_group: &mut EventGroupBuilder,
    wasm: &[u8],
    abi: &[u8],
    init_msg: &[u8],
) -> Address {
    let mut msg: Vec<u8> = init_msg_signature();
    msg.extend(init_msg);

    event_group
        .call(CONTRACT_DEPLOYER, Shortname::from_u32(1))
        .from_original_sender()
        .argument(wasm.to_vec())
        .argument(abi.to_vec())
        .argument(msg.to_vec())
        .done();

    Address {
        address_type: AddressType::PublicContract,
        identifier: ctx.original_transaction[12..32].try_into().unwrap(),
    }
}

fn init_msg_signature() -> Vec<u8> {
    vec![0xff, 0xff, 0xff, 0xff, 0x0f]
}
