use crate::codama_rust::instructions::WithdrawTreasuryFees;
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
}

impl InstructionsHelper {
    /// Builds the admin-only instruction that sweeps treasury SOL surplus and curve-admin ZINC.
    pub fn withdraw_treasury_fees_instruction(
        inputs: WithdrawTreasuryFeesInstructionInputs,
    ) -> Instruction {
        let WithdrawTreasuryFeesInstructionInputs {
            admin,
            zinc_mint,
            admin_token_account,
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
        .instruction()
    }
}
