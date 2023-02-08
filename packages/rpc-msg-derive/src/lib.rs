use darling::FromDeriveInput;
use proc_macro::{self, TokenStream};
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, FieldsNamed};

#[derive(FromDeriveInput, Default)]
#[darling(attributes(rpc_msg), forward_attrs(allow, doc, cfg))]
struct RpcMsgOpts {
    action: u32,
}

#[proc_macro_derive(IntoShortnameRPCEvent, attributes(rpc_msg))]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);
    let action = RpcMsgOpts::from_derive_input(&input)
        .expect("Options must be provided")
        .action;

    let DeriveInput { ident, data, .. } = input;

    let mut arguments_stream = TokenStream2::default();
    if let syn::Data::Struct(s) = data {
        if let syn::Fields::Named(FieldsNamed { named, .. }) = s.fields {
            let fields = named.iter().map(|f| &f.ident);
            let ftypes = named.iter().map(|f| &f.ty);

            for (field, _) in fields.into_iter().zip(ftypes.into_iter()) {
                arguments_stream.extend::<TokenStream2>(quote! {
                    interaction = interaction.argument(self.#field.clone());
                })
            }
        } else {
            unimplemented!("This macro works only with named fields that belong to the struct.");
        }
    } else {
        unimplemented!("This macro works only with structs.");
    };

    quote! {
        impl IntoShortnameRPCEvent for #ident {
            fn action_shortname(&self) -> u32 {
                #action
            }

            fn as_interaction(
                &self,
                builder:&mut pbc_contract_common::events::EventGroupBuilder,
                dest:&Address
            ) {
                let mut interaction = builder.call(*dest, Shortname::from_u32(self.action_shortname()));
                #arguments_stream
                interaction.done();
            }
        }
    }
    .into()
}
