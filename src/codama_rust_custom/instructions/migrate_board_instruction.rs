use crate::codama_rust::instructions::MigrateBoard;
use crate::codama_rust_custom::instructions::InstructionsHelper;
use crate::codama_rust_custom::pda::PdaHelper;
use solana_instruction::Instruction;
use solana_pubkey::Pubkey;

/// Inputs required to build the board-account migration instruction.
pub struct MigrateBoardInstructionInputs {
    /// Admin signer that authorizes the board migration.
    pub admin: Pubkey,
}

impl InstructionsHelper {
    /// Builds one `migrate_board` instruction for the board singleton PDA.
    pub fn migrate_board_instruction(inputs: MigrateBoardInstructionInputs) -> Instruction {
        let MigrateBoardInstructionInputs { admin } = inputs;
        MigrateBoard {
            admin,
            config: PdaHelper::get_config_address(),
            board: PdaHelper::get_board_address(),
            system_program: PdaHelper::get_system_program_address(),
        }
        .instruction()
    }
}
