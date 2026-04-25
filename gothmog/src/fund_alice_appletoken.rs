use solana_address::Address;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{signature::Keypair, signer::Signer};
use spl_associated_token_account::{
    get_associated_token_address, instruction::create_associated_token_account,
};
use spl_token_interface::instruction::mint_to;

pub fn fund_alice_appletoken(
    rpc: &RpcClient,
    alice_wallet: &Keypair,
    apple_mint: &Keypair,
    creator: &Keypair,
) -> Address {
    // Create ATA instruction
    println!("Create alice apple ata");
    let create_alice_apple_ata_ix = create_associated_token_account(
        &alice_wallet.pubkey(),
        &alice_wallet.pubkey(),
        &apple_mint.pubkey(),
        &solana_address::Address::from(spl_token_interface::id()),
    );

    let create_alice_apple_ata_tx = solana_sdk::transaction::Transaction::new_signed_with_payer(
        &[create_alice_apple_ata_ix],
        Some(&creator.pubkey()),
        &[&creator, &alice_wallet],
        rpc.get_latest_blockhash().unwrap(),
    );

    rpc.send_and_confirm_transaction(&create_alice_apple_ata_tx)
        .unwrap();

    // Fund alice account with apple token
    println!("Fund alice account with apple token");
    let alice_apple_ata =
        get_associated_token_address(&alice_wallet.pubkey(), &apple_mint.pubkey());

    let mint_apple_alice_ix = mint_to(
        &spl_token_interface::id(),
        &apple_mint.pubkey(),
        &alice_apple_ata,
        &creator.pubkey(),
        &[],
        20,
    )
    .unwrap();

    let fund_alice_apple_ata_tx = solana_sdk::transaction::Transaction::new_signed_with_payer(
        &[mint_apple_alice_ix],
        Some(&creator.pubkey()),
        &[&creator],
        rpc.get_latest_blockhash().unwrap(),
    );

    rpc.send_and_confirm_transaction(&fund_alice_apple_ata_tx)
        .unwrap();

    alice_apple_ata
}
