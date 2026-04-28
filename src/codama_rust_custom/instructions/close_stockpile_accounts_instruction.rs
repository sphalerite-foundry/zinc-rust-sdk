use crate::codama_rust::instructions::CloseStockpileAccounts;
use crate::codama_rust_custom::instructions::InstructionsHelper;
use crate::codama_rust_custom::pda::PdaHelper;
use solana_instruction::Instruction;
use solana_pubkey::Pubkey;

/// Inputs needed to close one terminal stockpile cycle's rent accounts.
pub struct CloseStockpileAccountsInstructionInputs {
    /// Crank signer submitting the cleanup transaction.
    pub signer: Pubkey,
    /// Stockpile id whose per-cycle accounts are being closed.
    pub stockpile_id: u64,
}

impl InstructionsHelper {
    /// Builds the close-stockpile-accounts instruction with stockpile PDAs resolved.
    pub fn close_stockpile_accounts_instruction(
        inputs: CloseStockpileAccountsInstructionInputs,
    ) -> Instruction {
        let CloseStockpileAccountsInstructionInputs {
            signer,
            stockpile_id,
        } = inputs;
        CloseStockpileAccounts {
            signer,
            config: PdaHelper::get_config_address(),
            board: PdaHelper::get_board_address(),
            stockpile: PdaHelper::get_stockpile_address(stockpile_id),
            stockpile_secret: PdaHelper::get_stockpile_secret_address(stockpile_id),
        }
        .instruction()
    }
}
