// region:    --- Modules

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
use balance::get_balance_in_sol;
use transaction::create_transfer_account;

// endregion: --- Modules

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
}