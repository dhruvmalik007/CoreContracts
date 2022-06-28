use proc_macro2::{Ident, TokenStream};

static CLIENT_ABI_VERSION: [u8; 3] = [4, 0, 0];
static BINDER_ABI_VERSION: [u8; 3] = [5, 0, 0];

pub(crate) fn create_version_numbers() -> TokenStream {
    let mut result = create_client_version();
    result.extend(create_binder_version());
    result
}

fn create_client_version() -> TokenStream {
    let name = version_name("CLIENT", CLIENT_ABI_VERSION);
    let version_for_abi = version_for_abi("CLIENT", CLIENT_ABI_VERSION);
    quote! {
        #[doc = "PBC Version of the binary format used by blockchain clients."]
        #[doc = "This versions the format of the binary data that smart contract code read/write to the contract state and the binary data received/sent in transactions/events."]
        #[no_mangle]
        pub static #name : () = ();

        #version_for_abi
    }
}

fn create_binder_version() -> TokenStream {
    let name = version_name("BINDER", BINDER_ABI_VERSION);
    let version_for_abi = version_for_abi("BINDER", BINDER_ABI_VERSION);
    quote! {
        #[doc = "PBC Version of the binary format used by the PBC WASM binder."]
        #[doc = "This versions the format of the binary data that the PBC WASM binder reads when handling smart contracts."]
        #[no_mangle]
        pub static #name : () = ();

        #version_for_abi
    }
}

fn version_name(version_type: &str, version: [u8; 3]) -> Ident {
    let major = version[0];
    let minor = version[1];
    let patch = version[2];

    format_ident!(
        "__PBC_VERSION_{}_{}_{}_{}",
        version_type,
        major,
        minor,
        patch
    )
}

fn version_for_abi(version_type: &str, version: [u8; 3]) -> TokenStream {
    let major = version[0];
    let minor = version[1];
    let patch = version[2];

    let fn_name = format_ident!("__abi_pbc_version_{}", version_type.to_lowercase());
    quote! {
        #[cfg(feature = "abi")]
        #[doc = "ABI: Machine readable version"]
        #[automatically_derived]
        #[no_mangle]
        pub fn #fn_name() -> [u8; 3] {
            [#major, #minor, #patch]
        }
    }
}
