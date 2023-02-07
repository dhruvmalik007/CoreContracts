use pbc_contract_common::{
    address::{Address, ShortnameCallback},
    events::EventGroupBuilder,
};
use pbc_traits::ReadWriteRPC;

/// ## Description
/// This trait describes methods that must be implemented
/// in order to be able to convert a struct into rpc event
pub trait IntoShortnameRPCEvent {
    fn action_shortname(&self) -> u32;
    fn as_interaction(&self, builder: &mut EventGroupBuilder, dest: &Address);
}
pub trait IntoShortnameRPCEventWithCost{
    fn action_shortname(&self) -> u32;
    fn as_interaction(&self, builder: &mut EventGroupBuilder, dest: &Address,cost:u64);
}

/// ## Description
/// Creates a callback event and adds it to event group builder object
/// ## Params
/// * **builder** is an object of type [`EventGroupBuilder`]
///
/// * **callback_byte** is an object of type [`u32`]
///
/// * **msg** is an object of type [`T`]
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

#[cfg(test)]
mod tests {
    use super::*;

    use create_type_spec_derive::CreateTypeSpec;
    use pbc_contract_common::{
        address::{Address, AddressType, Shortname},
        events::EventGroup,
    };
    use read_write_rpc_derive::ReadWriteRPC;
    use rpc_msg_derive::IntoShortnameRPCEvent;
    use rpc_msg_derive_with_cost::IntoShortnameRPCEventWithCost;
    #[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
    pub struct TestTransferMsg {
        pub to: Address,
        pub amount: u128,
        pub memo: String,
        pub amounts: Vec<u128>,
    }
    #[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
    pub struct TestTransferMsg2 {
        pub to: Address,
        pub amount: u128,
        pub memo: String,
        pub amounts: Vec<u128>,
    }

    impl IntoShortnameRPCEventWithCost for TestTransferMsg {
        fn action_shortname(&self) -> u32 {
            0x01
        }
        fn as_interaction(
            &self,
            builder: &mut pbc_contract_common::events::EventGroupBuilder,
            dest: &Address,
            cost:u64
        ) {
            builder
                .call(*dest, Shortname::from_u32(self.action_shortname()))
                .argument(self.to.clone())
                .argument(self.amount.clone())
                .argument(self.memo.clone())
                .argument(self.amounts.clone())
                .with_cost(cost)
                .done();
        }
    }
    impl IntoShortnameRPCEvent for TestTransferMsg2 {
        fn action_shortname(&self) -> u32 {
            0x01
        }
        fn as_interaction(
            &self,
            builder: &mut pbc_contract_common::events::EventGroupBuilder,
            dest: &Address,            
        ) {
            builder
                .call(*dest, Shortname::from_u32(self.action_shortname()))
                .argument(self.to.clone())
                .argument(self.amount.clone())
                .argument(self.memo.clone())
                .argument(self.amounts.clone())                
                .done();
        }
    }
    #[derive(ReadWriteRPC, CreateTypeSpec, IntoShortnameRPCEventWithCost, Clone, PartialEq, Eq, Debug)]
    #[rpc_msg(action = 0x01)]
    pub struct TestTransferMsgDeriveWithCost {
        pub to: Address,
        pub amount: u128,
        pub memo: String,
        pub amounts: Vec<u128>,
    }

    #[derive(ReadWriteRPC, CreateTypeSpec, IntoShortnameRPCEvent, Clone, PartialEq, Eq, Debug)]
    #[rpc_msg(action = 0x01)]
    pub struct TestTransferMsgDerive {
        pub to: Address,
        pub amount: u128,
        pub memo: String,
        pub amounts: Vec<u128>,
    }
    #[test]
    fn test_derive_macro_with_cost() {
        fn mock_address(le: u8) -> Address {
            Address {
                address_type: AddressType::Account,
                identifier: [
                    le, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                    0u8, 0u8, 0u8, 0u8,
                ],
            }
        }

        let msg = TestTransferMsg {
            to: mock_address(1u8),
            amount: 100,
            memo: "memo".to_string(),
            amounts: vec![10],
        };

        let derive_msg = TestTransferMsgDeriveWithCost {
            to: msg.to.clone(),
            amount: msg.amount,
            memo: msg.memo.clone(),
            amounts: msg.amounts.clone(),
        };

        assert_eq!(msg.action_shortname(), derive_msg.action_shortname());

        let dest = mock_address(10u8);
        let mut eg = EventGroup::builder();
        msg.as_interaction(&mut eg, &dest,12000);

        let mut derive_eg = EventGroup::builder();
        derive_msg.as_interaction(&mut derive_eg, &dest,12000);

        assert_eq!(eg.build(), derive_eg.build());
    }
    #[test]
    fn test_derive_macro() {
        fn mock_address(le: u8) -> Address {
            Address {
                address_type: AddressType::Account,
                identifier: [
                    le, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                    0u8, 0u8, 0u8, 0u8,
                ],
            }
        }

        let msg = TestTransferMsg2 {
            to: mock_address(1u8),
            amount: 100,
            memo: "memo".to_string(),
            amounts: vec![10],
        };

        let derive_msg = TestTransferMsgDerive {
            to: msg.to.clone(),
            amount: msg.amount,
            memo: msg.memo.clone(),
            amounts: msg.amounts.clone(),
        };

        assert_eq!(msg.action_shortname(), derive_msg.action_shortname());

        let dest = mock_address(10u8);
        let mut eg = EventGroup::builder();
        msg.as_interaction(&mut eg, &dest);

        let mut derive_eg = EventGroup::builder();
        derive_msg.as_interaction(&mut derive_eg, &dest);

        assert_eq!(eg.build(), derive_eg.build());
    }
}
