use crate::codama_rust::instructions::Migrate;
use crate::codama_rust_custom::instructions::InstructionsHelper;
use crate::codama_rust_custom::pda::PdaHelper;
use solana_instruction::Instruction;
use solana_pubkey::Pubkey;

/// Inputs for building one generic account migration instruction.
pub struct MigrateInstructionInputs {
    /// Admin signer authorized by the config account.
    pub admin: Pubkey,
    /// Zinc-owned account to migrate.
    pub account: Pubkey,
}

impl InstructionsHelper {
    /// Builds one `migrate` instruction for a registered Zinc-owned account.
    pub fn migrate_instruction(inputs: MigrateInstructionInputs) -> Instruction {
        let MigrateInstructionInputs { admin, account } = inputs;
        Migrate {
            admin,
            config: PdaHelper::get_config_address(),
            account,
            system_program: PdaHelper::get_system_program_address(),
        }
        .instruction()
    }
}
