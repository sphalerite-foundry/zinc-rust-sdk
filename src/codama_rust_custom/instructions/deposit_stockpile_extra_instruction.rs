use crate::codama_rust::instructions::{
    DepositStockpileExtra, DepositStockpileExtraInstructionArgs,
};
use crate::codama_rust_custom::instructions::InstructionsHelper;
use crate::codama_rust_custom::pda::PdaHelper;
use solana_instruction::Instruction;
use solana_pubkey::Pubkey;

pub struct DepositStockpileExtraInstructionInputs {
    /// Configured admin that funds the extra prize.
    pub admin: Pubkey,
    /// Live stockpile cycle receiving the extra prize.
    pub stockpile_id: u64,
    /// Classic SPL mint being deposited as an extra prize.
    pub extra_mint: Pubkey,
    /// Admin-owned classic SPL source token account.
    pub admin_source_token_account: Pubkey,
    /// Raw token amount to move into stockpile-extra custody.
    pub amount_raw: u64,
}

impl InstructionsHelper {
    /// Builds the `deposit_stockpile_extra` instruction from derived singleton accounts.
    pub fn deposit_stockpile_extra_instruction(
        inputs: DepositStockpileExtraInstructionInputs,
    ) -> Instruction {
        let DepositStockpileExtraInstructionInputs {
            admin,
            stockpile_id,
            extra_mint,
            admin_source_token_account,
            amount_raw,
        } = inputs;
        let treasury = PdaHelper::get_treasury_address();
        DepositStockpileExtra {
            admin,
            config: PdaHelper::get_config_address(),
            board: PdaHelper::get_board_address(),
            stockpile: PdaHelper::get_stockpile_address(stockpile_id),
            stockpile_extras: PdaHelper::get_stockpile_extras_address(),
            treasury,
            extra_mint,
            admin_source_token_account,
            stockpile_extra_token_account: PdaHelper::get_classic_ata(&treasury, &extra_mint),
            associated_token_program: PdaHelper::ASSOCIATED_TOKEN_PROGRAM_ID,
            token_program: PdaHelper::TOKEN_PROGRAM_ID,
            system_program: PdaHelper::get_system_program_address(),
        }
        .instruction(DepositStockpileExtraInstructionArgs { amount_raw })
    }
}
