use crate::codama_rust::instructions::CloseStockpile;
use crate::codama_rust_custom::instructions::InstructionsHelper;
use crate::codama_rust_custom::pda::PdaHelper;
use solana_instruction::Instruction;
use solana_pubkey::Pubkey;

pub struct CloseStockpileInstructionInputs {
    /// Authorized crank signer.
    pub signer: Pubkey,
    /// Stockpile cycle to close.
    pub stockpile_id: u64,
}

impl InstructionsHelper {
    pub fn close_stockpile_instruction(inputs: CloseStockpileInstructionInputs) -> Instruction {
        let CloseStockpileInstructionInputs {
            signer,
            stockpile_id,
        } = inputs;
        CloseStockpile {
            signer,
            config: PdaHelper::get_config_address(),
            board: PdaHelper::get_board_address(),
            stockpile: PdaHelper::get_stockpile_address(stockpile_id),
            treasury: PdaHelper::get_treasury_address(),
            stockpile_sol_vault: PdaHelper::get_stockpile_sol_vault_address(),
            stockpile_token_account: PdaHelper::get_stockpile_token_account_address(),
        }
        .instruction()
    }
}
