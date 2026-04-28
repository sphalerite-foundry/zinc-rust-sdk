use crate::codama_rust::instructions::{ClosePda, ClosePdaInstructionArgs};
use crate::codama_rust_custom::instructions::InstructionsHelper;
use crate::codama_rust_custom::pda::PdaHelper;
use solana_instruction::Instruction;
use solana_pubkey::Pubkey;

pub struct ClosePdaInstructionInputs {
    pub admin: Pubkey,
    pub account: Pubkey,
    pub allow_protected_close: bool,
}

impl InstructionsHelper {
    pub fn close_pda_instruction(inputs: ClosePdaInstructionInputs) -> Instruction {
        let ClosePdaInstructionInputs {
            admin,
            account,
            allow_protected_close,
        } = inputs;
        ClosePda {
            admin,
            account,
            config: PdaHelper::get_config_address(),
            system_program: PdaHelper::get_system_program_address(),
        }
        .instruction(ClosePdaInstructionArgs {
            allow_protected_close,
        })
    }
}
