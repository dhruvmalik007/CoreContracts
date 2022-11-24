use pbc_contract_common::{
    address::{Address, ShortnameCallback},
    events::EventGroupBuilder,
};
use pbc_traits::ReadWriteRPC;

pub trait IntoShortnameRPCEvent {
    fn action_shortname(&self) -> u32;
    fn as_interaction(&self, builder: &mut EventGroupBuilder, dest: &Address);
}

#[inline]
pub fn build_msg_callback<T>(builder: &mut EventGroupBuilder, callback_byte: u32, msg: &T)
where
    T: ReadWriteRPC + Clone,
{
    builder
        .with_callback(ShortnameCallback::from_u32(callback_byte))
        .argument(msg.clone())
        .done();
}
