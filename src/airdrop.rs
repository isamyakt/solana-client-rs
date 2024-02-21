use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::{ParseSignatureError, Signature};

use crate::balance::get_balance_in_lamports;

pub async fn airdrop(client: &RpcClient, pubkey: &Pubkey) -> Result<Signature, ParseSignatureError> {
    let balance = get_balance_in_lamports(client, pubkey).await;

    let lamports = 1000000000;

    if balance > lamports  {
        Err(ParseSignatureError::Invalid)
    } else  {
        let recent_blockhash = client
            .get_latest_blockhash()
            .unwrap();

        let airdrop_sig = client
            .request_airdrop_with_blockhash(&pubkey, lamports, &recent_blockhash)
            .unwrap();

        Ok(airdrop_sig)
    }
} 