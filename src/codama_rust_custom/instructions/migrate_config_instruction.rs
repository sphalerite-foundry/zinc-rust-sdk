use crate::codama_rust::instructions::MigrateConfig;
use crate::codama_rust_custom::instructions::InstructionsHelper;
use crate::codama_rust_custom::pda::PdaHelper;
use solana_instruction::Instruction;
use solana_pubkey::Pubkey;

/// Inputs required to build the config-account migration instruction.
pub struct MigrateConfigInstructionInputs {
    /// Admin signer that authorizes the config migration.
    pub admin: Pubkey,
}

impl InstructionsHelper {
    /// Builds one `migrate_config` instruction for the config singleton PDA.
    pub fn migrate_config_instruction(inputs: MigrateConfigInstructionInputs) -> Instruction {
        let MigrateConfigInstructionInputs { admin } = inputs;
        MigrateConfig {
            admin,
            config: PdaHelper::get_config_address(),
            system_program: PdaHelper::get_system_program_address(),
        }
        .instruction()
    }
}
