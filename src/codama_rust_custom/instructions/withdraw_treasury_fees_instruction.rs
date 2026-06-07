use crate::codama_rust::instructions::{WithdrawTreasuryFees, WithdrawTreasuryFeesInstructionArgs};
use crate::codama_rust_custom::instructions::InstructionsHelper;
use crate::codama_rust_custom::pda::PdaHelper;
use solana_instruction::Instruction;
use solana_pubkey::Pubkey;

/// Inputs for building one withdraw-treasury-fees instruction.
pub struct WithdrawTreasuryFeesInstructionInputs {
    /// Admin signer that receives the swept SOL and ZINC.
    pub admin: Pubkey,
    /// Protocol ZINC mint routed through the curve-admin fee vault.
    pub zinc_mint: Pubkey,
    /// Admin ATA that receives the swept ZINC balance.
    pub admin_token_account: Pubkey,
    /// Exact SOL lamports to withdraw from treasury fee custody.
    pub sol_lamports: u64,
    /// Exact ZINC amount to withdraw from curve-admin fee custody.
    pub zinc_amount: u64,
}

#[cfg(test)]
#[path = "withdraw_treasury_fees_instruction_tests.rs"]
mod tests;

impl InstructionsHelper {
    /// Builds the admin-only instruction that sweeps treasury SOL surplus and curve-admin ZINC.
    pub fn withdraw_treasury_fees_instruction(
        inputs: WithdrawTreasuryFeesInstructionInputs,
    ) -> Instruction {
        let WithdrawTreasuryFeesInstructionInputs {
            admin,
            zinc_mint,
            admin_token_account,
            sol_lamports,
            zinc_amount,
        } = inputs;
        WithdrawTreasuryFees {
            admin,
            config: PdaHelper::get_config_address(),
            treasury: PdaHelper::get_treasury_address(),
            zinc_mint,
            curve_admin_token_account: PdaHelper::get_classic_ata(
                &PdaHelper::get_treasury_address(),
                &zinc_mint,
            ),
            admin_token_account,
            associated_token_program: PdaHelper::ASSOCIATED_TOKEN_PROGRAM_ID,
            token_program: PdaHelper::TOKEN_PROGRAM_ID,
            system_program: PdaHelper::get_system_program_address(),
        }
        .instruction(WithdrawTreasuryFeesInstructionArgs {
            sol_lamports,
            zinc_amount,
        })
    }
}
