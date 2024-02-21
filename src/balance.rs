use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;


pub async fn get_balance_f64(client: &RpcClient, pubkey: &Pubkey) -> f64 {
    let lamports = 1000000000 as f64;
    let balance = client.get_balance(&pubkey).unwrap() as f64;

    balance / lamports
}

pub async fn get_balance_in_lamports(client: &RpcClient, pubkey: &Pubkey) -> u64 {
    let balance = client.get_balance(&pubkey).unwrap();
    balance
}