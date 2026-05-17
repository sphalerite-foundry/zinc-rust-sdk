use crate::codama_rust::instructions::CloseAutoMinerMiner;
use crate::codama_rust_custom::instructions::InstructionsHelper;
use crate::codama_rust_custom::pda::PdaHelper;
use solana_instruction::Instruction;
use solana_pubkey::Pubkey;

pub struct CloseAutoMinerMinerInstructionInputs {
    /// Crank signer submitting the cleanup transaction.
    pub signer: Pubkey,
    /// Player whose auto-created miner PDA is being closed.
    pub player: Pubkey,
    /// Auto-miner session PDA receiving the reclaimed miner rent.
    pub auto_miner_session: Pubkey,
    /// Round id that owns the miner account.
    pub round_id: u64,
}

impl InstructionsHelper {
    /// Builds the close-auto-miner-miner instruction with PDAs resolved.
    pub fn close_auto_miner_miner_instruction(
        inputs: CloseAutoMinerMinerInstructionInputs,
    ) -> Instruction {
        let CloseAutoMinerMinerInstructionInputs {
            signer,
            player,
            auto_miner_session,
            round_id,
        } = inputs;
        CloseAutoMinerMiner {
            signer,
            config: PdaHelper::get_config_address(),
            round: PdaHelper::get_round_address(round_id),
            auto_miner_session,
            miner: PdaHelper::get_miner_address(round_id, &player),
        }
        .instruction()
    }
}
