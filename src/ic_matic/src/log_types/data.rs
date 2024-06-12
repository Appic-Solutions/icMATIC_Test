use std::fmt::{Debug, Display, Formatter, LowerHex, UpperHex};

// Mostly used for converting address parameter in topics array to an address as wekll as data parameter in the logs
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct FixedSizeData(pub [u8; 32]);

impl AsRef<[u8]> for FixedSizeData {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl std::str::FromStr for FixedSizeData {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.starts_with("0x") {
            return Err("Ethereum hex string doesn't start with 0x".to_string());
        }
        let mut bytes = [0u8; 32];
        hex::decode_to_slice(&s[2..], &mut bytes)
            .map_err(|e: hex::FromHexError| format!("failed to decode hash from hex: {}", e))?;
        Ok(Self(bytes))
    }
}

impl Debug for FixedSizeData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:x}", self)
    }
}

impl Display for FixedSizeData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:x}", self)
    }
}

impl LowerHex for FixedSizeData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "0x{}", hex::encode(self.0))
    }
}

impl UpperHex for FixedSizeData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "0x{}", hex::encode_upper(self.0))
    }
}
