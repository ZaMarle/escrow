use serde::{Deserialize, Serialize};
use solana_address::Address;
use solana_sdk::signature::Keypair;

#[derive(Serialize, Deserialize)]
pub struct Wallet {
    pub pubkey: Address,
    pub secret: String,
}

impl Wallet {
    pub fn keypair(&self) -> Keypair {
        Keypair::from_base58_string(&self.secret)
    }
}
