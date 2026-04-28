use crate::codama_rust::instructions::{QueueStockpileReveal, QueueStockpileRevealInstructionArgs};
use crate::codama_rust_custom::instructions::InstructionsHelper;
use crate::codama_rust_custom::pda::PdaHelper;
use solana_instruction::Instruction;
use solana_pubkey::Pubkey;

pub struct QueueStockpileRevealInstructionInputs {
    pub signer: Pubkey,
    pub stockpile_id: u64,
    pub cluster_offset: u32,
    pub computation_offset: u64,
}

impl InstructionsHelper {
    pub fn queue_stockpile_reveal_instruction(
        inputs: QueueStockpileRevealInstructionInputs,
    ) -> Instruction {
        let QueueStockpileRevealInstructionInputs {
            signer,
            stockpile_id,
            cluster_offset,
            computation_offset,
        } = inputs;
        QueueStockpileReveal {
            signer,
            config: PdaHelper::get_config_address(),
            board: PdaHelper::get_board_address(),
            stockpile: PdaHelper::get_stockpile_address(stockpile_id),
            stockpile_secret: PdaHelper::get_stockpile_secret_address(stockpile_id),
            sign_pda_account: PdaHelper::get_sign_pda_account_address(),
            mxe_account: PdaHelper::get_mxe_account_address(),
            mempool_account: PdaHelper::get_mempool_account_address(cluster_offset),
            executing_pool: PdaHelper::get_executing_pool_address(cluster_offset),
            computation_account: PdaHelper::get_computation_account_address(
                cluster_offset,
                computation_offset,
            ),
            comp_def_account: PdaHelper::get_reveal_stockpile_rand_comp_def_account_address(),
            cluster_account: PdaHelper::get_cluster_account_address(cluster_offset),
            pool_account: PdaHelper::get_pool_account_address(),
            clock_account: PdaHelper::get_clock_account_address(),
            system_program: PdaHelper::get_system_program_address(),
            arcium_program: PdaHelper::ARCIUM_PROGRAM_ID,
        }
        .instruction(QueueStockpileRevealInstructionArgs { computation_offset })
    }
}
