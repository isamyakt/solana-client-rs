use solana_client::rpc_client::RpcClient;
use solana_sdk::signer::keypair::Keypair;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signer::Signer;
use solana_sdk::signature::Signature;
use solana_sdk::system_instruction;
use solana_sdk::transaction::Transaction;

pub async fn create_transfer_account(
    client: &RpcClient,
    sender: &Keypair,
    reciever: &Pubkey,
    amount: u64
) -> Signature {
    let instr = system_instruction::transfer(
        &sender.pubkey(),
        &reciever,
        amount,
    );

    let recent_blockhash = client.get_latest_blockhash().unwrap();
    
    let transaction = Transaction::new_signed_with_payer(
        &[instr], 
        Some(sender.pubkey()).as_ref(), 
        &[sender], 
        recent_blockhash
    );

    let sig = client.send_and_confirm_transaction(&transaction).unwrap();

    sig

}