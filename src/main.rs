use dotenvy::dotenv;
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signer::Signer;
use std::str::FromStr;

mod keypair;
mod airdrop;
mod balance;
mod transaction;

use keypair::initialize_keypair;
use airdrop::airdrop;
use balance::get_balance_f64;
use transaction::create_transfer_account;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let keypair = initialize_keypair().await;

    println!("{:?}", keypair);

    let public_key = keypair.pubkey();

    let url = String::from("https://api.devnet.solana.com");
    let client = RpcClient::new(url); 

    let airdrop = airdrop(&client, &public_key).await.unwrap();

    println!("airdrop sig : {}", airdrop);

    let balance = get_balance_f64(&client, &public_key).await;

    println!("balance: {} SOL", balance);

    let reciever = Pubkey::from_str("EtGf3KRUT2R21mAPCyZBXb7GFQy1sAeAfwBsHtCeBXP8").unwrap();
    let lamports = 100000;


    let tx = create_transfer_account(&client, &keypair, &reciever, lamports).await;

    println!("tx : {}", tx);

    let balance = get_balance_f64(&client, &public_key).await;

    println!("balance : {}", balance);
    
}
