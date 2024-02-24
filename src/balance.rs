// region:    --- Modules

use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::native_token::LAMPORTS_PER_SOL;

// endregion: --- Modules

// region:    --- Balance

pub async fn get_balance_in_sol(client: &RpcClient, pubkey: &Pubkey) -> f64 {
    let lamports = LAMPORTS_PER_SOL as f64;
    let balance = client.get_balance(&pubkey).unwrap() as f64;

    balance / lamports
}

pub async fn get_balance_in_lamports(client: &RpcClient, pubkey: &Pubkey) -> u64 {
    let balance = client.get_balance(&pubkey).unwrap();
    balance
}

// endregion:    --- Balance