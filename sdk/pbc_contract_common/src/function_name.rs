use sha2::{Digest, Sha256};

#[cfg(feature = "abi")]
use pbc_traits::ReadWriteRPC;
#[cfg(feature = "abi")]
use read_write_rpc_derive::ReadWriteRPC;

#[cfg(feature = "abi")]
use crate::abi::AbiSerialize;
use crate::to_leb128_bytes;

/// A small struct that automatically calculates the shortname of a function.
///
/// Serialized with the ABI format.
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "abi", derive(ReadWriteRPC))]
pub struct FunctionName {
    name: String,
    shortname: Shortname,
}

impl FunctionName {
    /// Create a new instance with the specified name. The shortname is calculated if None is
    /// supplied.
    pub fn new(name: String, shortname: Option<Shortname>) -> FunctionName {
        FunctionName::create_from_str(&name, shortname)
    }

    /// Create a new instance with the specified name as a str. The shortname is calculated eagerly.
    pub fn create_from_str(name: &str, shortname_override: Option<Shortname>) -> FunctionName {
        let shortname = if let Some(value) = shortname_override {
            value
        } else {
            name_to_shortname(name)
        };

        FunctionName {
            name: name.to_string(),
            shortname,
        }
    }

    /// Gets the Shortname
    pub fn shortname(&self) -> &Shortname {
        &self.shortname
    }
}

/// Denotes the kind of the ABI function hook.
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
#[cfg_attr(feature = "abi", derive(ReadWriteRPC))]
#[repr(u8)]
pub enum FunctionKind {
    /// Kind for `init` hook.
    Init = 0x01,
    /// Kind for `action` hook.
    Action = 0x02,
    /// Kind for `callback` hook.
    Callback = 0x03,
    /// Kind for `zk_on_secret_input` hook.
    ZkSecretInput = 0x10,
    /// Kind for `zk_on_variable_inputted` hook.
    ZkVarInputted = 0x11,
    /// Kind for `zk_on_variable_rejected` hook.
    ZkVarRejected = 0x12,
    /// Kind for `zk_on_compute_complete` hook.
    ZkComputeComplete = 0x13,
    /// Kind for `zk_on_variable_opened` hook.
    ZkVarOpened = 0x14,
    /// Kind for `zk_on_user_variable_opened` hook.
    ZkUserVarOpened = 0x15,
}

#[cfg(feature = "abi")]
impl AbiSerialize for FunctionName {
    fn serialize_abi<T: std::io::Write>(&self, writer: &mut T) -> std::io::Result<()> {
        self.name.rpc_write_to(writer)?;
        self.shortname.rpc_write_to(writer)
    }
}

/// Container for a LEB128-encoded shortname.
///
/// Instances of this type is always valid LEB128-encoded.
#[derive(Eq, PartialEq, Debug)]
pub struct Shortname {
    /// Invariants:
    /// - At least one byte long.
    /// - Last byte is less than 0x80.
    /// - Preceding bytes are larger than 0x80.
    bytes: Vec<u8>,
    /// Value
    value: u32,
}

impl Shortname {
    /// Create Shortname from an u32
    pub fn from_u32(value: u32) -> Self {
        Self {
            bytes: to_leb128_bytes(value),
            value,
        }
    }

    /// Create Shortname from a slice of bytes. Slice must be valid LEB128-encoded.
    pub fn from_be_bytes(bytes: &[u8]) -> Result<Self, String> {
        // Errors for last byte
        match bytes.last() {
            None => {
                return Result::Err("Shortname must not be empty".to_string());
            }
            Some(&b) if b >= 0x80 => {
                return Result::Err(
                    "Shortname's last byte must not have continuation bit set".to_string(),
                );
            }
            Some(&b) if b == 0x00 && bytes.len() > 1 => {
                return Result::Err(
                    "Shortname must be normalized, with no trailing zeroes".to_string(),
                );
            }
            _ => {} // Good
        }

        // Global validation
        let all_non_last_bytes_possess_continuation_bit =
            bytes.iter().rev().skip(1).all(|&b| b >= 0x80);
        if !all_non_last_bytes_possess_continuation_bit {
            return Result::Err(
                "Shortname's non-last bytes must have their continuation bits set".to_string(),
            );
        }

        let value_bytes: Vec<_> = bytes
            .iter()
            .enumerate()
            .map(|(i, &b)| actual_checked_shl(b as u32 & 0x7F, i as u32 * 7))
            .collect();

        if value_bytes.iter().any(|x| x.is_none()) {
            return Result::Err("Shortname value too large for u32".to_string());
        }

        Result::Ok(Self {
            bytes: bytes.to_vec(),
            value: value_bytes.iter().map(|x| x.unwrap()).sum(),
        })
    }

    /// Gets the shortname as it's u32 representation.
    ///
    /// Note invariant:
    ///
    /// ```
    /// # use pbc_contract_common::Shortname;
    /// # let i = 1231;
    /// assert_eq!(i, Shortname::from_u32(i).as_u32());
    /// ```
    pub fn as_u32(&self) -> u32 {
        self.value
    }
}

fn actual_checked_shl(lhs: u32, rhs: u32) -> Option<u32> {
    lhs.checked_shl(rhs).filter(|result| result >> rhs == lhs)
}

impl std::fmt::Display for Shortname {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for byte in &self.bytes {
            write!(f, "{:02x}", byte)?;
        }
        std::fmt::Result::Ok(())
    }
}

#[cfg(feature = "abi")]
impl ReadWriteRPC for Shortname {
    fn rpc_read_from<R: std::io::Read>(_reader: &mut R) -> Self {
        unimplemented!();
    }

    fn rpc_write_to<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        for item in &self.bytes {
            item.rpc_write_to(writer)?;
        }

        Ok(())
    }
}

/// Create a shortname from the given function name.
/// The shortname consists of the first 4 bytes of the SHA256 hash of the name.
fn name_to_shortname(raw_name: &str) -> Shortname {
    let mut digest = Sha256::new();
    Digest::update(&mut digest, raw_name.as_bytes());
    let output = digest.finalize();
    let first_four = output.chunks(4).next().unwrap();
    let shortname_u32 = u32::from_be_bytes(first_four.try_into().unwrap());
    Shortname::from_u32(shortname_u32)
}

#[cfg(all(test, feature = "abi"))]
mod test_abi_serialization {
    use crate::Shortname;

    fn interesting_shortname_values() -> Vec<(u32, Vec<u8>)> {
        return vec![
            (0, vec![0x00]),
            (1, vec![0x01]),
            (127, vec![0x7F]),
            (128, vec![0x80, 0x01]),
            (256, vec![0x80, 0x02]),
            (1000, vec![0xe8, 0x07]),
            (586977299, vec![0x93, 0xA0, 0xF2, 0x97, 0x02]),
        ];
    }

    #[test]
    fn shortname_as_u32() {
        for (i, shortname_bytes) in interesting_shortname_values() {
            let shortname = Shortname::from_be_bytes(&shortname_bytes).unwrap();
            assert_eq!(Shortname::from_u32(i), shortname);
            assert_eq!(shortname.as_u32(), i);
        }
    }

    #[test]
    fn u32_as_shortname_as_u32() {
        for (i, _) in interesting_shortname_values() {
            let shortname = Shortname::from_u32(i);
            assert_eq!(i, shortname.as_u32());
        }
    }

    #[test]
    fn u32_as_shortname_bytes_as_u32() {
        for (shortname_value, shortname_bytes) in interesting_shortname_values() {
            let parsed = Shortname::from_be_bytes(&shortname_bytes).unwrap();
            assert_eq!(shortname_bytes, parsed.bytes);
            assert_eq!(shortname_value, parsed.value);
        }
    }

    #[test]
    fn invalid_shortnames() {
        let invalid_shortname_bytes = [
            vec![0x00, 0x00],
            vec![0x00, 0x01],
            vec![0x00, 0x7F],
            vec![0x00, 0x80, 0x01],
            vec![0x80, 0x02, 0x00],
            vec![0x80],
            vec![0x80, 0x00], // Technically valid LEB128, but not normalized
            vec![0x80, 0x80, 0x80, 0x80, 0x32], // Too large for u32
            vec![0x93, 0xA0, 0xF2, 0x97, 0x32], // Too large for u32
        ];
        for bytes in invalid_shortname_bytes {
            let result = Shortname::from_be_bytes(&bytes);
            assert!(result.is_err(), "Must not succeed for bytes: {:?}", &bytes);
        }
    }
}
