use std::{collections::HashMap, fs};

use solana_client::rpc_client::RpcClient;

use crate::{
    create_bananacoin::create_bananacoin, create_usdc::create_usdc,
    fund_alice_appletoken::fund_alice_appletoken, fund_bob_bananatoken::fund_bob_bananatoken,
    fund_usdc::fund_usdc, wallet::Wallet,
};
mod create_applecoin;
mod create_bananacoin;
mod create_usdc;
mod fund_alice_appletoken;
mod fund_bob_bananatoken;
mod fund_usdc;
mod token;
mod wallet;

fn main() {
    println!("Starting gothmog");

    let rpc = RpcClient::new("http://127.0.0.1:8899".to_string());

    let wallets_json = fs::read_to_string("db/wallets.json").unwrap();
    let wallets: HashMap<String, Wallet> = serde_json::from_str(&wallets_json).unwrap();

    let sauron = wallets.get("sauron").unwrap();
    rpc.request_airdrop(&sauron.pubkey, 1_000_000_000).unwrap();

    let alice = wallets.get("alice").unwrap();
    rpc.request_airdrop(&alice.pubkey, 1_000_000_000).unwrap();

    let bob = wallets.get("bob").unwrap();
    rpc.request_airdrop(&bob.pubkey, 1_000_000_000).unwrap();

    // let mut wallets: HashMap<String, Wallet> = HashMap::new();
    // // sauron for creating accounts
    // let sauron = Keypair::new();
    // let sauron_wallet = Wallet {
    //     pubkey: sauron.pubkey(),
    //     secret: bs58::encode(sauron.to_bytes()).into_string(),
    // };
    // wallets.insert("sauron".to_string(), sauron_wallet);

    // // Create alice account
    // let alice = Keypair::new();
    // let alice_wallet = Wallet {
    //     pubkey: alice.pubkey(),
    //     secret: bs58::encode(alice.to_bytes()).into_string(),
    // };
    // wallets.insert("alice".to_string(), alice_wallet);

    // // Create bob account
    // let bob = Keypair::new();
    // let bob_wallet = Wallet {
    //     pubkey: bob.pubkey(),
    //     secret: bs58::encode(bob.to_bytes()).into_string(),
    // };
    // wallets.insert("bob".to_string(), bob_wallet);

    // fs::write(
    //     "db/wallets.json",
    //     serde_json::to_string_pretty(&wallets).unwrap(),
    // )
    // .unwrap();

    loop {
        let sb = rpc.get_balance(&sauron.pubkey).unwrap();
        let ab = rpc.get_balance(&alice.pubkey).unwrap();
        let bb = rpc.get_balance(&bob.pubkey).unwrap();
        if sb > 0 && ab > 0 && bb > 0 {
            break;
        };
        std::thread::sleep(std::time::Duration::from_millis(50));
    }

    let apple_mint = create_applecoin::create_applecoin(&rpc, &sauron.keypair());
    let banana_mint = create_bananacoin(&rpc, &sauron.keypair());
    let usdc_mint = create_usdc(&rpc, &sauron.keypair());

    let alice_apple_ata =
        fund_alice_appletoken(&rpc, &alice.keypair(), &apple_mint, &sauron.keypair());
    let bob_banana_ata =
        fund_bob_bananatoken(&rpc, &bob.keypair(), &banana_mint, &sauron.keypair());

    println!("Fund alice with USDC");
    let alice_usdc_ata = fund_usdc(&rpc, &alice.keypair(), &usdc_mint, &sauron.keypair(), 100);
    println!("Fund bob with USDC");
    let bob_usdc_ata = fund_usdc(&rpc, &bob.keypair(), &usdc_mint, &sauron.keypair(), 100);

    // Log state of alice account (all tokens owned)
    println!("Get alice applecoins");
    let alice_applecoins = rpc.get_token_account_balance(&alice_apple_ata);
    println!(
        "Alice has a balance of {} apple coins",
        alice_applecoins.unwrap().amount
    );

    // log bob
    println!("Get bob bananatokens");
    let bob_bananatokens = rpc.get_token_account_balance(&bob_banana_ata);
    println!(
        "Bob has a balance of {} banana tokens",
        bob_bananatokens.unwrap().amount
    );

    let alice_usdc = rpc.get_token_account_balance(&alice_usdc_ata).unwrap();
    println!("Alice has {} USDC", alice_usdc.ui_amount_string);

    let bob_usdc = rpc.get_token_account_balance(&bob_usdc_ata).unwrap();
    println!("Bob has {} USDC", bob_usdc.ui_amount_string);
}

// // 1. Setup in-memory validator
// // 2. Create Alice & Bob wallets
// // 3. Create AppleCoin & BananaCoin mints
// // 4. Mint 10 BananaCoin to Alice, 20 AppleCoin to Bob
// // 5. Alice calls initialize_escrow:
// //    - Temp PDA created
// //    - Escrow state PDA created
// //    - 10 BananaCoin moved → temp PDA
// // 6. Bob calls exchange:
// //    - 20 AppleCoin → Alice
// //    - 10 BananaCoin → Bob
// //    - PDAs closed
// // 7. Assert balances & account closure
