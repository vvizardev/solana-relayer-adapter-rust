use solana_sdk::{
    hash::Hash,
    instruction::Instruction,
    message::{VersionedMessage, v0::Message},
    pubkey::Pubkey,
    signature::Keypair,
    transaction::VersionedTransaction,
};

pub fn build_v0(
    mut ixs: Vec<Instruction>,
    fee_payer: &Pubkey,
    signers: &Vec<&Keypair>,
    recent_blockhash: Hash,
    nonce_ix: Option<Instruction>,
) -> String {
    if let Some(nonce_instruction) = nonce_ix {
        ixs.insert(0, nonce_instruction);
    }

    let message = Message::try_compile(fee_payer, &ixs, &[], recent_blockhash)
        .expect("Failed to compile message");
    let versioned_message = VersionedMessage::V0(message);
    let txn = VersionedTransaction::try_new(versioned_message, signers)
        .expect("Failed to create transaction");

    let serialized_tx = bincode::serialize(&txn).expect("Failed to serialize transaction");
    bs64::encode(&serialized_tx)
}

pub trait TransactionBuilder {
    fn build_v0(
        &self,
        ixs: Vec<Instruction>,
        fee_payer: &Pubkey,
        signers: &Vec<&Keypair>,
        recent_blockhash: Hash,
        nonce_ix: Option<Instruction>,
    ) -> String;
}
