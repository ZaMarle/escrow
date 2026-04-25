use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Token {
    pub name: String,
    pub mint: String,
    pub decimals: u8,
}
