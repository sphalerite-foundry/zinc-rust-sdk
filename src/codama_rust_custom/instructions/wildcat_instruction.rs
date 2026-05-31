use crate::codama_rust::instructions::{
    CLAIM_WILDCAT_DISCRIMINATOR, SELECT_WILDCAT_WINNER_DISCRIMINATOR,
};
use crate::codama_rust::ZINC_ID;
use crate::codama_rust_custom::instructions::InstructionsHelper;
use crate::codama_rust_custom::pda::PdaHelper;
use solana_instruction::{AccountMeta, Instruction};
use solana_pubkey::Pubkey;

/// Inputs for the deterministic Wildcat winner-selection crank instruction.
pub struct SelectWildcatWinnerInstructionInputs {
    /// Crank signer submitting the selection transaction.
    pub signer: Pubkey,
    /// Round id whose Wildcat draw should be scanned.
    pub round_id: u64,
    /// Whether to pass the per-round Wildcat sidecar PDA.
    pub include_round_wildcat_entries: bool,
}

/// Inputs for the crank-only Wildcat payout transfer instruction.
pub struct ClaimWildcatInstructionInputs {
    /// Crank signer submitting the Wildcat claim transaction.
    pub signer: Pubkey,
    /// Round id whose selected Wildcat payout should be transferred.
    pub round_id: u64,
    /// Selected winner wallet stored by the round account.
    pub winner: Pubkey,
    /// Protocol ZINC mint persisted on the treasury account.
    pub zinc_mint: Pubkey,
}

impl InstructionsHelper {
    /// Builds the deterministic Wildcat winner-selection instruction.
    pub fn select_wildcat_winner_instruction(
        inputs: SelectWildcatWinnerInstructionInputs,
    ) -> Instruction {
        let SelectWildcatWinnerInstructionInputs {
            signer,
            round_id,
            include_round_wildcat_entries,
        } = inputs;
        let mut accounts = vec![
            AccountMeta::new(signer, true),
            AccountMeta::new_readonly(PdaHelper::get_config_address(), false),
            AccountMeta::new(PdaHelper::get_round_address(round_id), false),
        ];
        if include_round_wildcat_entries {
            accounts.push(AccountMeta::new(
                PdaHelper::get_round_wildcat_entries_address(round_id),
                false,
            ));
        }
        Instruction {
            program_id: ZINC_ID,
            accounts,
            data: SELECT_WILDCAT_WINNER_DISCRIMINATOR.to_vec(),
        }
    }

    /// Builds the crank-only Wildcat payout transfer instruction.
    pub fn claim_wildcat_instruction(inputs: ClaimWildcatInstructionInputs) -> Instruction {
        let ClaimWildcatInstructionInputs {
            signer,
            round_id,
            winner,
            zinc_mint,
        } = inputs;
        let treasury = PdaHelper::get_treasury_address();
        let round_zinc_payout_token_account =
            PdaHelper::get_round_zinc_payout_token_account_address(round_id, &treasury, &zinc_mint);
        Instruction {
            program_id: ZINC_ID,
            accounts: vec![
                AccountMeta::new(signer, true),
                AccountMeta::new_readonly(PdaHelper::get_config_address(), false),
                AccountMeta::new(PdaHelper::get_round_address(round_id), false),
                AccountMeta::new_readonly(treasury, false),
                AccountMeta::new_readonly(zinc_mint, false),
                AccountMeta::new(round_zinc_payout_token_account, false),
                AccountMeta::new(winner, false),
                AccountMeta::new(PdaHelper::get_classic_ata(&winner, &zinc_mint), false),
                AccountMeta::new_readonly(PdaHelper::ASSOCIATED_TOKEN_PROGRAM_ID, false),
                AccountMeta::new_readonly(PdaHelper::TOKEN_PROGRAM_ID, false),
                AccountMeta::new_readonly(PdaHelper::get_system_program_address(), false),
            ],
            data: CLAIM_WILDCAT_DISCRIMINATOR.to_vec(),
        }
    }
}
