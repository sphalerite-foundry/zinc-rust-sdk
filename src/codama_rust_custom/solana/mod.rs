pub mod account;
pub mod lut;
pub mod transaction;

pub struct SolanaHelper;

pub fn to_sdk_pubkey(address: solana_address::Address) -> solana_sdk::pubkey::Pubkey {
    solana_sdk::pubkey::Pubkey::new_from_array(address.to_bytes())
}

pub fn from_sdk_pubkey(pubkey: solana_sdk::pubkey::Pubkey) -> solana_pubkey::Pubkey {
    solana_pubkey::Pubkey::new_from_array(pubkey.to_bytes())
}

pub fn to_sdk_instruction(
    instruction: solana_instruction::Instruction,
) -> solana_sdk::instruction::Instruction {
    solana_sdk::instruction::Instruction {
        program_id: to_sdk_pubkey(instruction.program_id),
        accounts: instruction
            .accounts
            .into_iter()
            .map(|account| solana_sdk::instruction::AccountMeta {
                pubkey: to_sdk_pubkey(account.pubkey),
                is_signer: account.is_signer,
                is_writable: account.is_writable,
            })
            .collect(),
        data: instruction.data,
    }
}
