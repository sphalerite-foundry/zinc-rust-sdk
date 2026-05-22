use crate::codama_rust::instructions::{InitStockpile, InitStockpileInstructionArgs};
use crate::codama_rust_custom::instructions::InstructionsHelper;
use crate::codama_rust_custom::pda::PdaHelper;
use solana_instruction::Instruction;
use solana_pubkey::Pubkey;

pub struct InitStockpileInstructionInputs {
    /// Crank payer that funds the stockpile account allocation and queue transaction.
    pub payer: Pubkey,
    /// Stockpile id fetched from `board.next_stockpile_id`, used only to derive PDAs locally.
    pub stockpile_id: u64,
    /// Protocol ZINC mint persisted on the treasury account.
    pub zinc_mint: Pubkey,
    /// Treasury-owned stockpile ZINC vault persisted on the treasury account.
    pub stockpile_token_account: Pubkey,
    /// Arcium cluster offset used to derive the runtime accounts.
    pub cluster_offset: u32,
    /// Unique computation offset for the queued randomness job.
    pub computation_offset: u64,
}

#[cfg(test)]
#[path = "init_stockpile_instruction_tests.rs"]
mod tests;

impl InstructionsHelper {
    /// Builds the init-stockpile instruction with the treasury-stored ZINC vault account.
    pub fn init_stockpile_instruction(inputs: InitStockpileInstructionInputs) -> Instruction {
        let InitStockpileInstructionInputs {
            payer,
            stockpile_id,
            zinc_mint,
            stockpile_token_account,
            cluster_offset,
            computation_offset,
        } = inputs;
        let instruction = InitStockpile {
            payer,
            config: PdaHelper::get_config_address(),
            board: PdaHelper::get_board_address(),
            treasury: PdaHelper::get_treasury_address(),
            zinc_mint,
            stockpile: PdaHelper::get_stockpile_address(stockpile_id),
            stockpile_secret: PdaHelper::get_stockpile_secret_address(stockpile_id),
            stockpile_extras: PdaHelper::get_stockpile_extras_address(),
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
        };
        let mut instruction = instruction.instruction_with_remaining_accounts(
            InitStockpileInstructionArgs { computation_offset },
            &[solana_instruction::AccountMeta::new_readonly(
                PdaHelper::ASSOCIATED_TOKEN_PROGRAM_ID,
                false,
            )],
        );
        if let Some(account) = instruction
            .accounts
            .iter_mut()
            .find(|account| account.pubkey == PdaHelper::get_stockpile_extras_address())
        {
            account.is_writable = true;
        }
        instruction
    }
}
