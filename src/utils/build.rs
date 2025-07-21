use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    hash::Hash,
    instruction::Instruction,
    message::{VersionedMessage, v0::Message},
    pubkey::Pubkey,
    signature::Keypair,
    transaction::{Transaction, VersionedTransaction},
};

pub fn build_v0_bs64(
    mut ixs: Vec<Instruction>,
    fee_payer: &Pubkey,
    signers: &Vec<&Keypair>,
    recent_blockhash: Hash,
    nonce_ix: Option<Instruction>,
) -> String {
    if let Some(nonce_instruction) = nonce_ix {
        ixs.insert(0, nonce_instruction);
    }

    let message: Message = Message::try_compile(fee_payer, &ixs, &[], recent_blockhash)
        .expect("Failed to compile message");
    let versioned_message = VersionedMessage::V0(message);
    let txn = VersionedTransaction::try_new(versioned_message, signers)
        .expect("Failed to create transaction");

    let serialized_tx = bincode::serialize(&txn).expect("Failed to serialize transaction");
    bs64::encode(&serialized_tx)
}

pub fn simulate(
    mut ixs: Vec<Instruction>,
    fee_payer: &Pubkey,
    signers: &Vec<&Keypair>,
    recent_blockhash: Hash,
    nonce_ix: Option<Instruction>,
    rpc_endpoint: String,
) {
    if let Some(nonce_instruction) = nonce_ix {
        ixs.insert(0, nonce_instruction);
    }

    let txn = Transaction::new_signed_with_payer(&ixs, Some(&fee_payer), signers, recent_blockhash);

    let rpc_client = RpcClient::new_with_commitment(
        (rpc_endpoint).clone(),
        solana_sdk::commitment_config::CommitmentConfig::processed(),
    );

    let tx_log = rpc_client.simulate_transaction(&txn);

    println!("Simulate {:#?}", tx_log);
}

pub trait TransactionBuilder {
    fn build_v0_bs64(
        &self,
        ixs: Vec<Instruction>,
        fee_payer: &Pubkey,
        signers: &Vec<&Keypair>,
        recent_blockhash: Hash,
        nonce_ix: Option<Instruction>,
    ) -> String;

    fn simulate(
        &self,
        ixs: Vec<Instruction>,
        fee_payer: &Pubkey,
        signers: &Vec<&Keypair>,
        recent_blockhash: Hash,
        nonce_ix: Option<Instruction>,
        rpc_endpoint: String,
    ) {
        simulate(
            ixs,
            fee_payer,
            signers,
            recent_blockhash,
            nonce_ix,
            rpc_endpoint,
        );
    }
}
