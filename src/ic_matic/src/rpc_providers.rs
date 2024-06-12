use std::fmt::{Display, Formatter};

pub enum PolygonNetwork {
    Mainnet,
    Amoy,
}

impl PolygonNetwork {
    pub fn chain_id(&self) -> u64 {
        match self {
            PolygonNetwork::Mainnet => 1,
            PolygonNetwork::Amoy => 80002,
        }
    }
}

impl TryFrom<u64> for PolygonNetwork {
    type Error = String;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(PolygonNetwork::Mainnet),
            80002 => Ok(PolygonNetwork::Amoy),
            _ => Err("Unknown Ethereum Network".to_string()),
        }
    }
}

impl Display for PolygonNetwork {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PolygonNetwork::Mainnet => write!(f, "Ethereum Mainnet"),
            PolygonNetwork::Amoy => write!(f, "Ethereum Testnet Sepolia"),
        }
    }
}
