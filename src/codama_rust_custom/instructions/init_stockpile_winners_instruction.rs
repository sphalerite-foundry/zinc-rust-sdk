use crate::codama_rust::ZINC_ID;
use crate::codama_rust_custom::instructions::InstructionsHelper;
use crate::codama_rust_custom::pda::PdaHelper;
use solana_instruction::{AccountMeta, Instruction};
use solana_pubkey::Pubkey;

/// Inputs for allocating ranked Stockpile winner storage after stockpile init.
pub struct InitStockpileWinnersInstructionInputs {
    /// Crank payer that funds ranked winner storage.
    pub payer: Pubkey,
    /// Stockpile id used to derive the stockpile and winners PDAs.
    pub stockpile_id: u64,
}

impl InstructionsHelper {
    /// Builds the init-stockpile-winners instruction for an unresolved stockpile.
    pub fn init_stockpile_winners_instruction(
        inputs: InitStockpileWinnersInstructionInputs,
    ) -> Instruction {
        let InitStockpileWinnersInstructionInputs {
            payer,
            stockpile_id,
        } = inputs;
        Instruction {
            program_id: ZINC_ID,
            accounts: vec![
                AccountMeta::new(payer, true),
                AccountMeta::new_readonly(PdaHelper::get_config_address(), false),
                AccountMeta::new_readonly(PdaHelper::get_board_address(), false),
                AccountMeta::new_readonly(PdaHelper::get_stockpile_address(stockpile_id), false),
                AccountMeta::new(
                    PdaHelper::get_stockpile_winners_address(stockpile_id),
                    false,
                ),
                AccountMeta::new_readonly(PdaHelper::get_system_program_address(), false),
            ],
            data: vec![174, 106, 84, 155, 219, 146, 37, 247],
        }
    }
}
