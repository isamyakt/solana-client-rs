use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::{Keypair, Signature};
use solana_sdk::signer::Signer;
use solana_sdk::transaction::Transaction;
use solana_sdk::system_instruction;

pub async fn create_multi_tx_account(
    client: &RpcClient,
    sender: &Keypair,
    recievers: &[&Pubkey],
    amount: &[u64],
) -> Option<Signature> {
    let first_transfer_ix = system_instruction::transfer(
        &sender.pubkey(), 
        recievers[0], 
        amount[0]
    );

    let second_transfer_ix = system_instruction::transfer(
        &sender.pubkey(), 
        recievers[1], 
        amount[1]
    );

    let third_transfer_ix = system_instruction::transfer(
        &sender.pubkey(), 
        recievers[2], 
        amount[2]
    );

    let fourth_transfer_ix = system_instruction::transfer(
        &sender.pubkey(), 
        recievers[3], 
        amount[3]
    );

    let recent_blockhash = client.get_latest_blockhash().unwrap();

    let tx = Transaction::new_signed_with_payer(
        &[first_transfer_ix.clone(), second_transfer_ix.clone(), third_transfer_ix.clone(), fourth_transfer_ix, third_transfer_ix, second_transfer_ix, first_transfer_ix], 
        Some(sender.pubkey()).as_ref(), 
        &[sender], 
        recent_blockhash
    );

    let sig = client.send_and_confirm_transaction(&tx).unwrap();

    Some(sig)
}

