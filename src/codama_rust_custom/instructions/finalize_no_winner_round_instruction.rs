use crate::codama_rust_custom::instructions::InstructionsHelper;
use crate::codama_rust_custom::pda::PdaHelper;
use solana_instruction::{AccountMeta, Instruction};
use solana_pubkey::pubkey;
use solana_pubkey::Pubkey;

const FINALIZE_NO_WINNER_ROUND_DISCRIMINATOR: [u8; 8] = [208, 203, 217, 27, 206, 70, 45, 36];
const TOKEN_PROGRAM_ID: solana_pubkey::Pubkey =
    pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");

pub struct FinalizeNoWinnerRoundInstructionInputs {
    /// Crank signer submitting the finalization transaction.
    pub signer: Pubkey,
    /// Zero-winner round waiting on redirect finalization.
    pub round_id: u64,
    /// Protocol ZINC mint persisted on the treasury account.
    pub zinc_mint: Pubkey,
    /// Bonanza vault that receives the 70% fixed-ZINC redirect share.
    pub bonanza_token_account: Pubkey,
    /// Dedicated stockpile vault that receives the 30% fixed-ZINC redirect share.
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
            stockpile_token_account,
        } = inputs;
        let treasury = PdaHelper::get_treasury_address();
        let round_zinc_payout_token_account =
            PdaHelper::get_round_zinc_payout_token_account_address(round_id, &treasury, &zinc_mint);
        Instruction {
            program_id: crate::codama_rust::ZINC_ID,
            accounts: vec![
                AccountMeta::new(signer, true),
                AccountMeta::new_readonly(PdaHelper::get_config_address(), false),
                AccountMeta::new_readonly(PdaHelper::get_board_address(), false),
                AccountMeta::new(PdaHelper::get_round_address(round_id), false),
                AccountMeta::new(treasury, false),
                AccountMeta::new(PdaHelper::get_stockpile_sol_vault_address(), false),
                AccountMeta::new(zinc_mint, false),
                AccountMeta::new(round_zinc_payout_token_account, false),
                AccountMeta::new(bonanza_token_account, false),
                AccountMeta::new(stockpile_token_account, false),
                AccountMeta::new_readonly(
                    Pubkey::new_from_array(TOKEN_PROGRAM_ID.to_bytes()),
                    false,
                ),
            ],
            data: FINALIZE_NO_WINNER_ROUND_DISCRIMINATOR.to_vec(),
        }
    }
}
