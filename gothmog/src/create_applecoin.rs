use std::{collections::HashMap, fs};

use solana_client::rpc_client::RpcClient;
use solana_sdk::{program_pack::Pack, signature::Keypair, signer::Signer};
use solana_system_interface::instruction;

use crate::token::Token;

pub fn create_applecoin(rpc: &RpcClient, creator: &Keypair) -> Keypair {
    // Create apple token
    let apple_mint = Keypair::new();
    let decimals = 0;
    let token = Token {
        decimals,
        name: "AppleCoin".to_string(),
        mint: apple_mint.pubkey().to_string(),
    };
    let mut tokens: HashMap<String, Token> =
        serde_json::from_str(&fs::read_to_string("db/tokens.json").unwrap()).unwrap();
    tokens.insert("AppleCoin".to_string(), token);
    fs::write(
        "db/tokens.json",
        serde_json::to_string_pretty(&tokens).unwrap(),
    )
    .unwrap();

    let rent = rpc
        .get_minimum_balance_for_rent_exemption(spl_token_interface::state::Mint::LEN)
        .unwrap();

    // Create account for mint
    let create_apple_mint_ix = instruction::create_account(
        &creator.pubkey(),
        &apple_mint.pubkey(),
        rent,
        spl_token_interface::state::Mint::LEN as u64,
        &solana_address::Address::from(spl_token_interface::id()),
    );

    // Init mint
    let init_apple_mint_ix = spl_token_interface::instruction::initialize_mint(
        &spl_token_interface::id(),
        &apple_mint.pubkey(),
        &creator.pubkey(),
        None,
        decimals,
    )
    .unwrap();

    println!("Create AppleCoin");
    let tx = solana_sdk::transaction::Transaction::new_signed_with_payer(
        &[create_apple_mint_ix, init_apple_mint_ix],
        Some(&creator.pubkey()),
        &[&creator, &apple_mint],
        rpc.get_latest_blockhash().unwrap(),
    );

    rpc.send_and_confirm_transaction(&tx).unwrap();

    apple_mint
}
