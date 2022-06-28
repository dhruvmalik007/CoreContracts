use pbc_contract_common::{to_leb128_bytes, FunctionName};
use pbc_traits::ReadWriteRPC;

pub trait NamedRPCEvent {
    fn event_name(&self) -> String;
}

#[inline]
pub fn into_rpc_call<T>(msg: T) -> Vec<u8>
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
pub fn into_shortname_rpc_call<T>(msg: T) -> Vec<u8>
where
    T: ShortnamedRPCEvent + ReadWriteRPC,
{
    let mut event_payload: Vec<u8> = msg.short_event_name();
    msg.rpc_write_to(&mut event_payload).unwrap();

    event_payload
}
