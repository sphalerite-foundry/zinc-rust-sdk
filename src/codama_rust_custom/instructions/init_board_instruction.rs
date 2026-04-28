use crate::codama_rust::instructions::InitBoard;
use crate::codama_rust_custom::instructions::InstructionsHelper;
use crate::codama_rust_custom::pda::PdaHelper;
use solana_instruction::Instruction;
use solana_pubkey::Pubkey;

pub struct InitBoardInstructionInputs {
    pub authority: Pubkey,
}

impl InstructionsHelper {
    pub fn init_board_instruction(inputs: InitBoardInstructionInputs) -> Instruction {
        let InitBoardInstructionInputs { authority } = inputs;
        InitBoard {
            authority,
            config: PdaHelper::get_config_address(),
            board: PdaHelper::get_board_address(),
            system_program: PdaHelper::get_system_program_address(),
        }
        .instruction()
    }
}
