// region:    --- Modules

use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;
use solana_sdk::native_token::LAMPORTS_PER_SOL;

use crate::balance::get_balance_in_lamports;

// endregion: --- Modules


// region:    --- Airdrop

pub async fn airdrop_possible(client: &RpcClient, pubkey: &Pubkey) -> bool {
    let balance = get_balance_in_lamports(client, pubkey).await;

    let result = !(balance > LAMPORTS_PER_SOL);
    result
}

pub async fn airdrop(client: &RpcClient, pubkey: &Pubkey) -> Option<Signature> {
    let airdrop_available = airdrop_possible(client, pubkey).await;

    if airdrop_available  {
        let recent_blockhash = client
            .get_latest_blockhash()
            .unwrap();

        let lamports = 1000000000;

        let airdrop_sig = client
            .request_airdrop_with_blockhash(&pubkey, lamports, &recent_blockhash)
            .unwrap();

        return Some(airdrop_sig);
    } 

    None
} 

// endregion: --- Airdrop