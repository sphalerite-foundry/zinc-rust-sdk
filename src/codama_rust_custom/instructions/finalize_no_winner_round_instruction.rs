use crate::codama_rust::instructions::FinalizeNoWinnerRound;
use crate::codama_rust_custom::instructions::InstructionsHelper;
use crate::codama_rust_custom::pda::PdaHelper;
use solana_instruction::Instruction;
use solana_pubkey::pubkey;
use solana_pubkey::Pubkey;

const TOKEN_PROGRAM_ID: solana_pubkey::Pubkey =
    pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");

#[cfg(test)]
#[path = "finalize_no_winner_round_instruction_tests.rs"]
mod tests;

pub struct FinalizeNoWinnerRoundInstructionInputs {
    /// Crank signer submitting the finalization transaction.
    pub signer: Pubkey,
    /// Zero-winner round waiting on redirect finalization.
    pub round_id: u64,
    /// Protocol ZINC mint persisted on the treasury account.
    pub zinc_mint: Pubkey,
    /// Bonanza vault that receives the configured Bonanza redirect remainder.
    pub bonanza_token_account: Pubkey,
    /// Liquidity vault that receives the configured Bonanza redirect split.
    pub liquidity_zinc_token_account: Pubkey,
    /// Dedicated stockpile vault that receives the configured stockpile redirect share.
    pub stockpile_token_account: Pubkey,
}

impl InstructionsHelper {
    /// Builds the post-settlement zero-winner finalization instruction.
    pub fn finalize_no_winner_round_instruction(
        inputs: FinalizeNoWinnerRoundInstructionInputs,
    ) -> Instruction {
        let FinalizeNoWinnerRoundInstructionInputs {
            signer,
            round_id,
            zinc_mint,
            bonanza_token_account,
            liquidity_zinc_token_account,
            stockpile_token_account,
        } = inputs;
        let treasury = PdaHelper::get_treasury_address();
        let round_zinc_payout_token_account =
            PdaHelper::get_round_zinc_payout_token_account_address(round_id, &treasury, &zinc_mint);
        FinalizeNoWinnerRound {
            signer,
            config: PdaHelper::get_config_address(),
            board: PdaHelper::get_board_address(),
            round: PdaHelper::get_round_address(round_id),
            treasury,
            stockpile_sol_vault: PdaHelper::get_stockpile_sol_vault_address(),
            zinc_mint,
            round_zinc_payout_token_account,
            bonanza_token_account,
            liquidity_zinc_token_account,
            stockpile_token_account,
            token_program: Pubkey::new_from_array(TOKEN_PROGRAM_ID.to_bytes()),
        }
        .instruction()
    }
}
