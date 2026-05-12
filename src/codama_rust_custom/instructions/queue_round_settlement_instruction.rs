use crate::codama_rust::instructions::{QueueRoundSettlement, QueueRoundSettlementInstructionArgs};
use crate::codama_rust_custom::instructions::InstructionsHelper;
use crate::codama_rust_custom::pda::PdaHelper;
use solana_instruction::Instruction;
use solana_pubkey::Pubkey;

pub struct QueueRoundSettlementInstructionInputs {
    /// Authority signing the reveal queue transaction.
    pub signer: Pubkey,
    /// Round whose encrypted secret should be revealed.
    pub round_id: u64,
    /// Arcium cluster offset used to derive queue accounts.
    pub cluster_offset: u32,
    /// Per-computation Arcium account offset for this reveal.
    pub computation_offset: u64,
}

impl InstructionsHelper {
    pub fn queue_round_settlement_instruction(
        inputs: QueueRoundSettlementInstructionInputs,
    ) -> Instruction {
        let QueueRoundSettlementInstructionInputs {
            signer,
            round_id,
            cluster_offset,
            computation_offset,
        } = inputs;
        QueueRoundSettlement {
            signer,
            config: PdaHelper::get_config_address(),
            board: PdaHelper::get_board_address(),
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
            comp_def_account: PdaHelper::get_reveal_round_rand_comp_def_account_address(),
            cluster_account: PdaHelper::get_cluster_account_address(cluster_offset),
            pool_account: PdaHelper::get_pool_account_address(),
            clock_account: PdaHelper::get_clock_account_address(),
            system_program: PdaHelper::get_system_program_address(),
            arcium_program: PdaHelper::ARCIUM_PROGRAM_ID,
        }
        .instruction(QueueRoundSettlementInstructionArgs { computation_offset })
    }
}
