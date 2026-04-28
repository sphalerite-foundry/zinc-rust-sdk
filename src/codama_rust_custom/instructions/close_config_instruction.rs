use crate::codama_rust::instructions::CloseConfig;
use crate::codama_rust_custom::instructions::InstructionsHelper;
use crate::codama_rust_custom::pda::PdaHelper;
use solana_instruction::Instruction;
use solana_pubkey::Pubkey;

/// Inputs for building one close-config instruction.
pub struct CloseConfigInstructionInputs {
    /// Admin signer that receives the closed config account rent.
    pub admin: Pubkey,
}

impl InstructionsHelper {
    /// Builds the admin-only instruction that closes the singleton config PDA.
    pub fn close_config_instruction(inputs: CloseConfigInstructionInputs) -> Instruction {
        let CloseConfigInstructionInputs { admin } = inputs;
        CloseConfig {
            admin,
            config: PdaHelper::get_config_address(),
        }
        .instruction()
    }
}
