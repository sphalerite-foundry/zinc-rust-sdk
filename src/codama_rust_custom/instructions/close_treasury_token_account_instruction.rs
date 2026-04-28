use crate::codama_rust::instructions::CloseTreasuryTokenAccount;
use crate::codama_rust_custom::instructions::InstructionsHelper;
use crate::codama_rust_custom::pda::PdaHelper;
use solana_instruction::Instruction;
use solana_pubkey::Pubkey;

/// Inputs for building one close-treasury-token-account instruction.
pub struct CloseTreasuryTokenAccountInstructionInputs {
    /// Admin signer that receives drained tokens and closed-account rent.
    pub admin: Pubkey,
    /// Mint stored on the treasury-owned source token account.
    pub mint: Pubkey,
    /// Treasury-owned token account to drain and close.
    pub source_token_account: Pubkey,
    /// Admin ATA that receives the drained token balance.
    pub admin_token_account: Pubkey,
}

impl InstructionsHelper {
    /// Builds the admin-only instruction that drains and closes one treasury token account.
    pub fn close_treasury_token_account_instruction(
        inputs: CloseTreasuryTokenAccountInstructionInputs,
    ) -> Instruction {
        let CloseTreasuryTokenAccountInstructionInputs {
            admin,
            mint,
            source_token_account,
            admin_token_account,
        } = inputs;
        CloseTreasuryTokenAccount {
            admin,
            config: PdaHelper::get_config_address(),
            treasury: PdaHelper::get_treasury_address(),
            mint,
            source_token_account,
            admin_token_account,
            associated_token_program: PdaHelper::ASSOCIATED_TOKEN_PROGRAM_ID,
            token_program: PdaHelper::TOKEN_PROGRAM_ID,
            system_program: PdaHelper::get_system_program_address(),
        }
        .instruction()
    }
}
