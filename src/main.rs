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

    println!("{:?}\n", keypair);

    let public_key = keypair.pubkey();

    let url = String::from("https://api.devnet.solana.com");
    let client = RpcClient::new(url); 

    let airdrop = airdrop(&client, &public_key).await;

    if let Some(sig) = airdrop {
        println!("airdrop signature : {}\n", sig);
    } 
    println!("airdrop failed : becoz your balance is more then 1 SOL\n");


    let balance = get_balance_f64(&client, &public_key).await;

    println!("before balance : {} SOL\n", balance);

    let reciever = Pubkey::from_str("EtGf3KRUT2R21mAPCyZBXb7GFQy1sAeAfwBsHtCeBXP8").unwrap();
    let lamports = 10000000; // 0.01 SOL


    let tx = create_transfer_account(&client, &keypair, &reciever, lamports).await;

    if let Some(sig) = tx {
        println!("tx : {}\n", sig);
    } 

    let balance = get_balance_f64(&client, &public_key).await;

    println!("after balance : {}\n", balance);
    
}
