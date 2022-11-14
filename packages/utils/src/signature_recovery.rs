use pbc_contract_common::{address::Address, address::AddressType, events::EventGroupBuilder};
use libsecp256k1::{recover, sign, Message, PublicKeyFormat, RecoveryId, SecretKey, Signature};
pub fn address_to_bytes(input: Address) -> [u8; 21] {
    let address_type_value: u8 = match input.address_type {
        AddressType::Account => 0u8,
        AddressType::SystemContract => 1u8,
        AddressType::PublicContract => 2u8,
        AddressType::ZkContract => 3u8,
    };
    let mut address: [u8; 21] = [0; 21];
    address[0] = address_type_value;
    address[1..21].copy_from_slice(&input.identifier);
    address
}
/// ## Description
/// utility function to recover a public key from a signature
///  ## Params
    
/// * **sig_buf* is an field of type [u8;65]
/// * **digest* is an field of type [u8;32]
pub fn recover_public_key_from_signature(sig_buf: [u8; 65], digest: [u8; 32]) -> [u8; 33] {
    // Partisia Signature is 65 bytes
    let message = Message::parse(&digest);
    let signature = Signature::parse_standard(&sig_buf[1..65].try_into().unwrap()).unwrap();
    let recovery_id = RecoveryId::parse(sig_buf[0]).unwrap();
    let public_key = recover(&message, &signature, &recovery_id).unwrap();

    public_key.serialize_compressed()
}
/// ## Description
/// utility function to  convert a public key to address 
///  ## Params
    
/// * **public_key* is an field of type [u8;33]
   
pub fn public_key_to_address(public_key: [u8; 33]) -> [u8; 21] {
    // partisia address is the sha256 hash of the uncompressed public key
    let public_key_uncompressed: [u8; 65] = libsecp256k1::PublicKey::parse_slice(
        public_key.as_slice(),
        Some(PublicKeyFormat::Compressed),
    )
    .unwrap()
    .serialize();
    
    let h: [u8; 32] = Sha3_256::digest(&public_key_uncompressed)
        .try_into()
        .unwrap();
    

    let mut address: [u8; 21] = [0; 21];
    address[1..21].copy_from_slice(&h[12..32]);
    address
}
/// ## Description
/// utility function to recover a public key from the signature of hashed data and convert it to an address
///  ## Params
    
/// * **sig_buf* is an field of type [u8;65]
/// * **digest* is an field of type [u8;32]
pub fn recover_address_identifier(sig_buf: [u8; 65], digest: [u8; 32]) -> [u8; 21] {
    let raw_key = recover_public_key_from_signature(sig_buf, digest);
    public_key_to_address(raw_key)
}