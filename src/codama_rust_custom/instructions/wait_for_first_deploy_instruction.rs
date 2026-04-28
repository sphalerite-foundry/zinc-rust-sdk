use crate::codama_rust::instructions::WaitForFirstDeploy;
use crate::codama_rust_custom::instructions::InstructionsHelper;
use crate::codama_rust_custom::pda::PdaHelper;
use solana_instruction::Instruction;
use solana_pubkey::Pubkey;

pub struct WaitForFirstDeployInstructionInputs {
    pub signer: Pubkey,
    pub round_id: u64,
}

impl InstructionsHelper {
    pub fn wait_for_first_deploy_instruction(
        inputs: WaitForFirstDeployInstructionInputs,
    ) -> Instruction {
        let WaitForFirstDeployInstructionInputs { signer, round_id } = inputs;
        WaitForFirstDeploy {
            signer,
            config: PdaHelper::get_config_address(),
            board: PdaHelper::get_board_address(),
            round: PdaHelper::get_round_address(round_id),
            round_secret: PdaHelper::get_round_secret_address(round_id),
        }
        .instruction()
    }
}
