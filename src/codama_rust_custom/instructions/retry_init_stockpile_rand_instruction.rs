use crate::codama_rust::instructions::{
    RetryInitStockpileRand, RetryInitStockpileRandInstructionArgs,
};
use crate::codama_rust_custom::instructions::InstructionsHelper;
use crate::codama_rust_custom::pda::PdaHelper;
use solana_instruction::Instruction;
use solana_pubkey::Pubkey;

pub struct RetryInitStockpileRandInstructionInputs {
    /// Crank payer that funds the retry queue transaction.
    pub payer: Pubkey,
    /// Existing unresolved stockpile id to retry.
    pub stockpile_id: u64,
    /// Treasury-owned stockpile ZINC vault persisted on the treasury account.
    pub stockpile_token_account: Pubkey,
    /// Arcium cluster offset used to derive the runtime accounts.
    pub cluster_offset: u32,
    /// Unique computation offset for the queued randomness job.
    pub computation_offset: u64,
}

#[cfg(test)]
#[path = "retry_init_stockpile_rand_instruction_tests.rs"]
mod tests;

impl InstructionsHelper {
    /// Builds the retry-init-stockpile-rand instruction for an existing preparing stockpile.
    pub fn retry_init_stockpile_rand_instruction(
        inputs: RetryInitStockpileRandInstructionInputs,
    ) -> Instruction {
        let RetryInitStockpileRandInstructionInputs {
            payer,
            stockpile_id,
            stockpile_token_account,
            cluster_offset,
            computation_offset,
        } = inputs;
        RetryInitStockpileRand {
            payer,
            config: PdaHelper::get_config_address(),
            board: PdaHelper::get_board_address(),
            treasury: PdaHelper::get_treasury_address(),
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
            comp_def_account: PdaHelper::get_init_stockpile_rand_comp_def_account_address(),
            cluster_account: PdaHelper::get_cluster_account_address(cluster_offset),
            pool_account: PdaHelper::get_pool_account_address(),
            clock_account: PdaHelper::get_clock_account_address(),
            stockpile_sol_vault: PdaHelper::get_stockpile_sol_vault_address(),
            stockpile_token_account,
            token_program: PdaHelper::TOKEN_PROGRAM_ID,
            system_program: PdaHelper::get_system_program_address(),
            arcium_program: PdaHelper::ARCIUM_PROGRAM_ID,
        }
        .instruction(RetryInitStockpileRandInstructionArgs {
            stockpile_id,
            computation_offset,
        })
    }
}
