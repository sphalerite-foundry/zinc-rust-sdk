use crate::codama_rust::instructions::{
    QueueSettleWinningStakesBatch, QueueSettleWinningStakesBatchInstructionArgs,
};
use crate::codama_rust_custom::instructions::InstructionsHelper;
use crate::codama_rust_custom::pda::PdaHelper;
use anyhow::anyhow;
use solana_instruction::{AccountMeta, Instruction};
use solana_pubkey::Pubkey;

const SETTLE_WINNING_STAKES_BATCH_SIZE: usize = 8;

pub struct QueueSettleWinningStakesBatchInstructionInputs {
    pub signer: Pubkey,
    pub round_id: u64,
    pub cluster_offset: u32,
    pub computation_offset: u64,
    pub miners: Vec<Pubkey>,
}

impl InstructionsHelper {
    pub fn queue_settle_winning_stakes_batch_instruction(
        inputs: QueueSettleWinningStakesBatchInstructionInputs,
    ) -> anyhow::Result<Instruction> {
        let QueueSettleWinningStakesBatchInstructionInputs {
            signer,
            round_id,
            cluster_offset,
            computation_offset,
            miners,
        } = inputs;
        if miners.is_empty() {
            return Err(anyhow!(
                "queue settle batch requires at least one miner account"
            ));
        }
        if miners.len() > SETTLE_WINNING_STAKES_BATCH_SIZE {
            return Err(anyhow!(
                "queue settle batch supports at most {SETTLE_WINNING_STAKES_BATCH_SIZE} miner accounts"
            ));
        }
        let remaining_accounts = miners
            .into_iter()
            .map(|miner| AccountMeta::new(miner, false))
            .collect::<Vec<_>>();
        Ok(QueueSettleWinningStakesBatch {
            signer,
            config: PdaHelper::get_config_address(),
            board: PdaHelper::get_board_address(),
            round: PdaHelper::get_round_address(round_id),
            round_secret: PdaHelper::get_round_secret_address(round_id),
            treasury: PdaHelper::get_treasury_address(),
            sign_pda_account: PdaHelper::get_sign_pda_account_address(),
            mxe_account: PdaHelper::get_mxe_account_address(),
            mempool_account: PdaHelper::get_mempool_account_address(cluster_offset),
            executing_pool: PdaHelper::get_executing_pool_address(cluster_offset),
            computation_account: PdaHelper::get_computation_account_address(
                cluster_offset,
                computation_offset,
            ),
            comp_def_account: PdaHelper::get_settle_winning_stakes_batch_comp_def_account_address(),
            cluster_account: PdaHelper::get_cluster_account_address(cluster_offset),
            pool_account: PdaHelper::get_pool_account_address(),
            clock_account: PdaHelper::get_clock_account_address(),
            system_program: PdaHelper::get_system_program_address(),
            arcium_program: PdaHelper::ARCIUM_PROGRAM_ID,
        }
        .instruction_with_remaining_accounts(
            QueueSettleWinningStakesBatchInstructionArgs { computation_offset },
            &remaining_accounts,
        ))
    }
}
