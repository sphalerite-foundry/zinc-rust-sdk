use crate::codama_rust::instructions::CloseRoundAccounts;
use crate::codama_rust_custom::instructions::InstructionsHelper;
use crate::codama_rust_custom::pda::PdaHelper;
use solana_instruction::Instruction;
use solana_pubkey::Pubkey;

pub struct CloseRoundAccountsInstructionInputs {
    /// Crank signer submitting the cleanup transaction.
    pub signer: Pubkey,
    /// Round id whose parent accounts are being closed.
    pub round_id: u64,
    /// Protocol ZINC mint persisted on the treasury account.
    pub zinc_mint: Pubkey,
    /// Curve-admin vault that receives residual round ZINC dust.
    pub curve_admin_token_account: Pubkey,
}

impl InstructionsHelper {
    /// Builds the close-round-accounts instruction with all singleton and round PDAs resolved.
    pub fn close_round_accounts_instruction(
        inputs: CloseRoundAccountsInstructionInputs,
    ) -> Instruction {
        let CloseRoundAccountsInstructionInputs {
            signer,
            round_id,
            zinc_mint,
            curve_admin_token_account,
        } = inputs;
        let treasury = PdaHelper::get_treasury_address();
        let round_zinc_payout_token_account =
            PdaHelper::get_round_zinc_payout_token_account_address(round_id, &treasury, &zinc_mint);
        CloseRoundAccounts {
            signer,
            config: PdaHelper::get_config_address(),
            board: PdaHelper::get_board_address(),
            round: PdaHelper::get_round_address(round_id),
            round_secret: PdaHelper::get_round_secret_address(round_id),
            treasury,
            zinc_mint,
            round_zinc_payout_token_account,
            curve_admin_token_account,
            token_program: PdaHelper::TOKEN_PROGRAM_ID,
        }
        .instruction()
    }
}
