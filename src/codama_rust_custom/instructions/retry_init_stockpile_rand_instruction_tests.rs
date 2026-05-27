use super::{InstructionsHelper, RetryInitStockpileRandInstructionInputs};
use crate::codama_rust_custom::pda::PdaHelper;
use solana_pubkey::Pubkey;

#[test]
fn retry_init_stockpile_rand_instruction_uses_explicit_stockpile_id() {
    let stockpile_id = 0;
    let computation_offset = 99;
    let stockpile_token_account = PdaHelper::get_stockpile_token_account_address();
    let instruction = InstructionsHelper::retry_init_stockpile_rand_instruction(
        RetryInitStockpileRandInstructionInputs {
            payer: Pubkey::new_unique(),
            stockpile_id,
            stockpile_token_account,
            cluster_offset: 2026,
            computation_offset,
        },
    );

    assert_eq!(instruction.accounts.len(), 20);
    assert_eq!(
        instruction.accounts[4].pubkey,
        PdaHelper::get_stockpile_address(stockpile_id)
    );
    assert!(instruction.accounts[4].is_writable);
    assert_eq!(
        instruction.accounts[5].pubkey,
        PdaHelper::get_stockpile_secret_address(stockpile_id)
    );
    assert!(instruction.accounts[5].is_writable);
    assert_eq!(
        instruction.accounts[10].pubkey,
        PdaHelper::get_computation_account_address(2026, computation_offset)
    );
    assert_eq!(instruction.accounts[16].pubkey, stockpile_token_account);
}
