use crate::codama_rust::instructions::{PayoutStockpile, PayoutStockpileInstructionArgs};
use crate::codama_rust_custom::instructions::InstructionsHelper;
use crate::codama_rust_custom::pda::PdaHelper;
use solana_instruction::Instruction;
use solana_pubkey::Pubkey;

/// Inputs for the crank-only Stockpile base payout instruction.
pub struct PayoutStockpileInstructionInputs {
    /// Authorized crank signer.
    pub signer: Pubkey,
    /// Stockpile cycle being paid out.
    pub stockpile_id: u64,
    /// Ranked winner slot to pay.
    pub rank: u8,
    /// Resolved stockpile winner that receives SOL and profile-credited ZINC.
    pub winner: Pubkey,
    /// Protocol ZINC mint persisted on the treasury account.
    pub zinc_mint: Pubkey,
}

impl InstructionsHelper {
    /// Builds the crank-only Stockpile base payout instruction.
    pub fn payout_stockpile_instruction(inputs: PayoutStockpileInstructionInputs) -> Instruction {
        let PayoutStockpileInstructionInputs {
            signer,
            stockpile_id,
            rank,
            winner,
            zinc_mint,
        } = inputs;
        PayoutStockpile {
            signer,
            config: PdaHelper::get_config_address(),
            stockpile: PdaHelper::get_stockpile_address(stockpile_id),
            stockpile_winners: PdaHelper::get_stockpile_winners_address(stockpile_id),
            stockpile_extras: PdaHelper::get_stockpile_extras_address(),
            board: PdaHelper::get_board_address(),
            treasury: PdaHelper::get_treasury_address(),
            stockpile_sol_vault: PdaHelper::get_stockpile_sol_vault_address(),
            zinc_mint,
            stockpile_token_account: PdaHelper::get_stockpile_token_account_address(),
            winner,
            winner_player_profile: PdaHelper::get_player_profile_address(&winner),
            round_zinc_reward_token_account: PdaHelper::get_round_zinc_reward_token_account_address(
            ),
            token_program: PdaHelper::TOKEN_PROGRAM_ID,
        }
        .instruction(PayoutStockpileInstructionArgs { rank })
    }
}
