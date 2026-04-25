use std::{collections::HashMap, fs};

use solana_client::rpc_client::RpcClient;
use solana_sdk::{program_pack::Pack, signature::Keypair, signer::Signer, transaction};
use solana_system_interface::instruction;

use crate::token::Token;

pub fn create_bananacoin(rpc: &RpcClient, creator: &Keypair) -> Keypair {
    let banana_mint = Keypair::new();
    let decimals = 0;
    let token = Token {
        decimals,
        name: "BananaCoin".to_string(),
        mint: banana_mint.pubkey().to_string(),
    };
    let mut tokens: HashMap<String, Token> =
        serde_json::from_str(&fs::read_to_string("db/tokens.json").unwrap()).unwrap();
    tokens.insert("BananaCoin".to_string(), token);
    fs::write(
        "db/tokens.json",
        serde_json::to_string_pretty(&tokens).unwrap(),
    )
    .unwrap();

    let rent = rpc
        .get_minimum_balance_for_rent_exemption(spl_token_interface::state::Mint::LEN)
        .unwrap();

    // Create instruction
    let create_ix = instruction::create_account(
        &creator.pubkey(),
        &banana_mint.pubkey(),
        rent,
        spl_token_interface::state::Mint::LEN as u64,
        &solana_address::Address::from(spl_token_interface::id()),
    );

    // Init instruction
    let init_ix = spl_token_interface::instruction::initialize_mint(
        &spl_token_interface::id(),
        &banana_mint.pubkey(),
        &creator.pubkey(),
        None,
        0,
    )
    .unwrap();

    // Transaction
    println!("Create BananaCoin");
    let tx = transaction::Transaction::new_signed_with_payer(
        &[create_ix, init_ix],
        Some(&creator.pubkey()),
        &[&creator, &banana_mint],
        rpc.get_latest_blockhash().unwrap(),
    );

    // Perform transaction
    rpc.send_and_confirm_transaction(&tx).unwrap();

    banana_mint
}
