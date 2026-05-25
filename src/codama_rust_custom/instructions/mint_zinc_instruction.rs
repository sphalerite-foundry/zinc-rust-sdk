use crate::codama_rust::instructions::{MintZinc, MintZincInstructionArgs};
use crate::codama_rust_custom::instructions::InstructionsHelper;
use crate::codama_rust_custom::pda::PdaHelper;
use solana_instruction::Instruction;
use solana_pubkey::Pubkey;

/// Inputs for building one admin ZINC mint instruction.
pub struct MintZincInstructionInputs {
    /// Configured admin signer that receives the minted canonical ZINC.
    pub admin: Pubkey,
    /// Protocol ZINC mint controlled by the treasury PDA.
    pub zinc_mint: Pubkey,
    /// Admin ATA that receives the newly minted ZINC.
    pub admin_token_account: Pubkey,
    /// Raw base-unit ZINC amount to mint.
    pub amount: u64,
}

impl InstructionsHelper {
    /// Builds the admin-only instruction that mints capped canonical ZINC to the admin ATA.
    pub fn mint_zinc_instruction(inputs: MintZincInstructionInputs) -> Instruction {
        let MintZincInstructionInputs {
            admin,
            zinc_mint,
            admin_token_account,
            amount,
        } = inputs;
        MintZinc {
            admin,
            config: PdaHelper::get_config_address(),
            treasury: PdaHelper::get_treasury_address(),
            zinc_mint,
            admin_token_account,
            associated_token_program: PdaHelper::ASSOCIATED_TOKEN_PROGRAM_ID,
            token_program: PdaHelper::TOKEN_PROGRAM_ID,
            system_program: PdaHelper::get_system_program_address(),
        }
        .instruction(MintZincInstructionArgs { amount })
    }
}
