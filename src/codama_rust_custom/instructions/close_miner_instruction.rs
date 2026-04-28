use crate::codama_rust::instructions::CloseMiner;
use crate::codama_rust_custom::instructions::InstructionsHelper;
use crate::codama_rust_custom::pda::PdaHelper;
use solana_instruction::Instruction;
use solana_pubkey::Pubkey;

pub struct CloseMinerInstructionInputs {
    /// Crank signer submitting the cleanup transaction.
    pub signer: Pubkey,
    /// Player whose miner PDA is being closed.
    pub player: Pubkey,
    /// Round id that owns the miner account.
    pub round_id: u64,
}

impl InstructionsHelper {
    /// Builds the close-miner instruction with round and miner PDAs resolved.
    pub fn close_miner_instruction(inputs: CloseMinerInstructionInputs) -> Instruction {
        let CloseMinerInstructionInputs {
            signer,
            player,
            round_id,
        } = inputs;
        CloseMiner {
            signer,
            config: PdaHelper::get_config_address(),
            round: PdaHelper::get_round_address(round_id),
            miner: PdaHelper::get_miner_address(round_id, &player),
            player,
        }
        .instruction()
    }
}
