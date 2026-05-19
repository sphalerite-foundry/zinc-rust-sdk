use crate::codama_rust::instructions::RevealRoundBlockhash;
use crate::codama_rust_custom::instructions::InstructionsHelper;
use crate::codama_rust_custom::pda::PdaHelper;
use solana_instruction::Instruction;
use solana_pubkey::{pubkey, Pubkey};

pub struct RevealRoundBlockhashInstructionInputs {
    /// Authority signing the blockhash reveal transaction.
    pub signer: Pubkey,
    /// Round whose delayed blockhash sample should be revealed.
    pub round_id: u64,
}

impl InstructionsHelper {
    pub fn reveal_round_blockhash_instruction(
        inputs: RevealRoundBlockhashInstructionInputs,
    ) -> Instruction {
        let RevealRoundBlockhashInstructionInputs { signer, round_id } = inputs;
        RevealRoundBlockhash {
            signer,
            config: PdaHelper::get_config_address(),
            board: PdaHelper::get_board_address(),
            round: PdaHelper::get_round_address(round_id),
            slot_hashes: pubkey!("SysvarS1otHashes111111111111111111111111111"),
        }
        .instruction()
    }
}
