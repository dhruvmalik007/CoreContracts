use pbc_contract_common::{
    address::{Address, Shortname},
    events::{EventGroupBuilder, InteractionBuilder},
    to_leb128_bytes, FunctionName,
};
use pbc_traits::ReadWriteRPC;

pub trait NamedRPCEvent {
    fn event_name(&self) -> String;
}

#[inline]
pub fn get_msg_shortname<T>(msg: &T) -> Shortname
where
    T: NamedRPCEvent + ReadWriteRPC,
{
    *FunctionName::create_from_str(msg.event_name().as_str(), None).shortname()
}

#[inline]
pub fn build_msg_call<T>(
    builder: &mut EventGroupBuilder,
    dest: &Address,
    from_original_sender: bool,
    msg: &T,
) where
    T: NamedRPCEvent + ReadWriteRPC + Clone,
{
    let mut interaction = builder.call(*dest, get_msg_shortname(msg));
    if from_original_sender {
        interaction = interaction.from_original_sender();
    }

    interaction.argument(msg.clone()).done();
}

#[inline]
pub fn into_rpc_call<T>(msg: &T) -> Vec<u8>
where
    T: NamedRPCEvent + ReadWriteRPC,
{
    let fn_name = FunctionName::create_from_str(msg.event_name().as_str(), None);
    let mut event_payload: Vec<u8> = to_leb128_bytes(fn_name.shortname().as_u32());
    msg.rpc_write_to(&mut event_payload).unwrap();

    event_payload
}

pub trait ShortnamedRPCEvent {
    fn short_event_name(&self) -> Vec<u8>;
}

#[inline]
pub fn into_shortname_rpc_call<T>(msg: &T) -> Vec<u8>
where
    T: ShortnamedRPCEvent + ReadWriteRPC,
{
    let mut event_payload: Vec<u8> = msg.short_event_name();
    msg.rpc_write_to(&mut event_payload).unwrap();

    event_payload
}

#[inline]
pub fn decode_base64_into_rpc_call(method_name: &str, payload: &str) -> Vec<u8> {
    let fn_name = FunctionName::create_from_str(method_name, None);
    let event_name: Vec<u8> = to_leb128_bytes(fn_name.shortname().as_u32());
    let event_payload = base64::decode(payload).unwrap();

    [event_name, event_payload].concat()
}
