use solana_address::Address;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{signature::Keypair, signer::Signer};
use spl_associated_token_account::{
    get_associated_token_address, instruction::create_associated_token_account,
};
use spl_token_interface::instruction::mint_to;

pub fn fund_bob_bananatoken(
    rpc: &RpcClient,
    bob_wallet: &Keypair,
    banana_mint: &Keypair,
    creator: &Keypair,
) -> Address {
    // Create ATA instruction
    println!("Create bob banana ata");
    let create_ata_ix = create_associated_token_account(
        &bob_wallet.pubkey(),
        &bob_wallet.pubkey(),
        &banana_mint.pubkey(),
        &solana_address::Address::from(spl_token_interface::id()),
    );

    let create_ata_tx = solana_sdk::transaction::Transaction::new_signed_with_payer(
        &[create_ata_ix],
        Some(&creator.pubkey()),
        &[&creator, &bob_wallet],
        rpc.get_latest_blockhash().unwrap(),
    );

    rpc.send_and_confirm_transaction(&create_ata_tx).unwrap();

    // Fund bob account with banana token
    println!("Fund bob account with banana token");
    let ata = get_associated_token_address(&bob_wallet.pubkey(), &banana_mint.pubkey());

    let mint_ix = mint_to(
        &solana_address::Address::from(spl_token_interface::id()),
        &banana_mint.pubkey(),
        &ata,
        &creator.pubkey(),
        &[],
        40,
    )
    .unwrap();

    let mint_tx = solana_sdk::transaction::Transaction::new_signed_with_payer(
        &[mint_ix],
        Some(&creator.pubkey()),
        &[&creator],
        rpc.get_latest_blockhash().unwrap(),
    );

    rpc.send_and_confirm_transaction(&mint_tx).unwrap();

    ata
}
