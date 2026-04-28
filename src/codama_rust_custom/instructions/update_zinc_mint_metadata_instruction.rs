use crate::codama_rust::instructions::{
    UpdateZincMintMetadata, UpdateZincMintMetadataInstructionArgs,
};
use crate::codama_rust::types::ZincMintMetadataArgs;
use crate::codama_rust_custom::instructions::InstructionsHelper;
use crate::codama_rust_custom::pda::PdaHelper;
use solana_instruction::Instruction;
use solana_pubkey::Pubkey;

/// Inputs needed to update the protocol ZINC mint Metaplex metadata.
pub struct UpdateZincMintMetadataInstructionInputs {
    /// Admin authorized by the protocol config.
    pub admin: Pubkey,
    /// Protocol ZINC mint whose metadata should be updated.
    pub zinc_mint: Pubkey,
    /// Replacement Metaplex metadata fields.
    pub metadata: ZincMintMetadataArgs,
}

impl InstructionsHelper {
    /// Builds the admin instruction that updates the ZINC mint metadata account.
    pub fn update_zinc_mint_metadata_instruction(
        inputs: UpdateZincMintMetadataInstructionInputs,
    ) -> Instruction {
        let UpdateZincMintMetadataInstructionInputs {
            admin,
            zinc_mint,
            metadata,
        } = inputs;
        let treasury = PdaHelper::get_treasury_address();
        let zinc_metadata = PdaHelper::get_mint_metadata_address(&zinc_mint);
        UpdateZincMintMetadata {
            admin,
            config: PdaHelper::get_config_address(),
            treasury,
            zinc_mint,
            zinc_metadata,
            metadata_program: PdaHelper::METADATA_PROGRAM_ID,
        }
        .instruction(UpdateZincMintMetadataInstructionArgs { args: metadata })
    }
}
