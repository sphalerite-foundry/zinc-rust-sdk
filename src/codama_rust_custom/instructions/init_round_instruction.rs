use crate::codama_rust::instructions::{InitRound, InitRoundInstructionArgs};
use crate::codama_rust_custom::instructions::InstructionsHelper;
use crate::codama_rust_custom::pda::PdaHelper;
use solana_instruction::{AccountMeta, Instruction};
use solana_pubkey::Pubkey;

pub struct InitRoundInstructionInputs {
    pub payer: Pubkey,
    pub round_id: u64,
    pub cluster_offset: u32,
    pub computation_offset: u64,
}

impl InstructionsHelper {
    pub fn init_round_instruction(inputs: InitRoundInstructionInputs) -> Instruction {
        let InitRoundInstructionInputs {
            payer,
            round_id,
            cluster_offset,
            computation_offset,
        } = inputs;
        let mut instruction = InitRound {
            payer,
            config: PdaHelper::get_config_address(),
            board: PdaHelper::get_board_address(),
            treasury: PdaHelper::get_treasury_address(),
            round: PdaHelper::get_round_address(round_id),
            round_secret: PdaHelper::get_round_secret_address(round_id),
            sign_pda_account: PdaHelper::get_sign_pda_account_address(),
            mxe_account: PdaHelper::get_mxe_account_address(),
            mempool_account: PdaHelper::get_mempool_account_address(cluster_offset),
            executing_pool: PdaHelper::get_executing_pool_address(cluster_offset),
            computation_account: PdaHelper::get_computation_account_address(
                cluster_offset,
                computation_offset,
            ),
            comp_def_account: PdaHelper::get_init_round_rand_comp_def_account_address(),
            cluster_account: PdaHelper::get_cluster_account_address(cluster_offset),
            pool_account: PdaHelper::get_pool_account_address(),
            clock_account: PdaHelper::get_clock_account_address(),
            system_program: PdaHelper::get_system_program_address(),
            arcium_program: PdaHelper::ARCIUM_PROGRAM_ID,
        }
        .instruction(InitRoundInstructionArgs {
            round_id,
            computation_offset,
        });
        instruction.accounts.push(AccountMeta::new(
            PdaHelper::get_round_wildcat_entries_address(round_id),
            false,
        ));
        instruction
    }
}
