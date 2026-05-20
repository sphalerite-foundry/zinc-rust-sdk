use crate::codama_rust::instructions::InitStockpileWinners;
use crate::codama_rust_custom::instructions::InstructionsHelper;
use crate::codama_rust_custom::pda::PdaHelper;
use solana_instruction::Instruction;
use solana_pubkey::Pubkey;

/// Inputs needed to build an init-stockpile-winners instruction.
pub struct InitStockpileWinnersInstructionInputs {
    /// Crank payer that funds the ranked winners account allocation.
    pub payer: Pubkey,
    /// Stockpile id used to derive the stockpile and winners PDAs locally.
    pub stockpile_id: u64,
}

#[cfg(test)]
#[path = "init_stockpile_winners_instruction_tests.rs"]
mod tests;

impl InstructionsHelper {
    /// Builds the init-stockpile-winners instruction for one closed stockpile cycle.
    pub fn init_stockpile_winners_instruction(
        inputs: InitStockpileWinnersInstructionInputs,
    ) -> Instruction {
        let InitStockpileWinnersInstructionInputs {
            payer,
            stockpile_id,
        } = inputs;
        InitStockpileWinners {
            payer,
            config: PdaHelper::get_config_address(),
            board: PdaHelper::get_board_address(),
            stockpile: PdaHelper::get_stockpile_address(stockpile_id),
            stockpile_winners: PdaHelper::get_stockpile_winners_address(stockpile_id),
            system_program: PdaHelper::get_system_program_address(),
        }
        .instruction()
    }
}
