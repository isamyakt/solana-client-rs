// region:    --- Modules

use std::str::FromStr;

use dotenvy::dotenv;
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
// use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
// use std::str::FromStr;

mod keypair;
mod airdrop;
mod balance;
mod transaction;
mod multi_txs;

use keypair::initialize_keypair;
use airdrop::airdrop;
use balance::get_balance_in_sol;
use transaction::create_transfer_account;
use multi_txs::create_multi_tx_account;

// endregion: --- Modules

async fn multi_tx(client: &RpcClient, keypair: &Keypair) {
    let recievers = [
        &Pubkey::from_str("EtGf3KRUT2R21mAPCyZBXb7GFQy1sAeAfwBsHtCeBXP8").unwrap(),
        &Pubkey::from_str("9o7acD8UP8DDKEDZ1LFzuajC7bwG2WZJXRdG1i5FAfD3").unwrap(),
        &Pubkey::from_str("G2MeMHLr84SbTWVfBj7HSLqPLNQqmR9T8Mkepxi2Ag8V").unwrap(),
        &Pubkey::from_str("7QnSXgoZHi9FGCwaziaEMsUtmWZUbuvg3qq5UCGVJFat").unwrap(),
    ];

    let amount_lamports: [u64; 4] = [
        1_000_000, 
        2_000_000, 
        3_000_000,
        4_000_000
    ];

    let multi_tx_sign = create_multi_tx_account(
        client, 
        &keypair, 
        &recievers, 
        &amount_lamports, 
    ).await;

    if let Some(sig) = multi_tx_sign {
        println!("multi tx : {}\n", sig);
    }
}


#[tokio::main]
async fn main() {
    dotenv().ok();

    let keypair = initialize_keypair().await;

    println!("{:?}\n", keypair);

    let public_key = keypair.pubkey();

    let url = String::from("https://api.devnet.solana.com");
    let client = RpcClient::new(url); 

    let airdrop = airdrop(&client, &public_key).await;

    if let Some(sig) = airdrop {
        println!("airdrop signature : {}\n", sig);
    } 
    println!("airdrop failed : becoz your balance is more then 1 SOL\n");


    let balance = get_balance_in_sol(&client, &public_key).await;

    println!("before balance : {} SOL\n", balance);

    let reciever = Pubkey::from_str("Fudp7uPDYNYQRxoq1Q4JiwJnzyxhVz37bGqRki3PBzS").unwrap();
    let transfer_lamports = 10000000; // 0.01 SOL


    let tx = create_transfer_account(&client, &keypair, &reciever, transfer_lamports).await;

    if let Some(sig) = tx {
        println!("tx : {}\n", sig);
    } 

    let balance = get_balance_in_sol(&client, &public_key).await;

    println!("after balance : {}\n", balance);

    multi_tx(&client, &keypair).await;

    let balance = get_balance_in_sol(&client, &public_key).await;

    println!("after multi-tx balance : {}\n", balance);

}