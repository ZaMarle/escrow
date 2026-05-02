use solana_address::Address;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{signature::Keypair, signer::Signer};
use spl_associated_token_account::{
    get_associated_token_address, instruction::create_associated_token_account,
};
use spl_token_interface::instruction::mint_to;

const USDC_DECIMALS: u32 = 6;

pub fn fund_usdc(
    rpc: &RpcClient,
    wallet: &Keypair,
    usdc_mint: &Keypair,
    creator: &Keypair,
    amount: u64,
) -> Address {
    let create_ata_ix = create_associated_token_account(
        &wallet.pubkey(),
        &wallet.pubkey(),
        &usdc_mint.pubkey(),
        &solana_address::Address::from(spl_token_interface::id()),
    );

    let create_ata_tx = solana_sdk::transaction::Transaction::new_signed_with_payer(
        &[create_ata_ix],
        Some(&creator.pubkey()),
        &[&creator, &wallet],
        rpc.get_latest_blockhash().unwrap(),
    );

    rpc.send_and_confirm_transaction(&create_ata_tx).unwrap();

    let ata = get_associated_token_address(&wallet.pubkey(), &usdc_mint.pubkey());

    let raw_amount = amount * 10u64.pow(USDC_DECIMALS);
    let mint_ix = mint_to(
        &spl_token_interface::id(),
        &usdc_mint.pubkey(),
        &ata,
        &creator.pubkey(),
        &[],
        raw_amount,
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
