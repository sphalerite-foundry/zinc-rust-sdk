use super::{InitStockpileInstructionInputs, InstructionsHelper};
use crate::codama_rust_custom::pda::PdaHelper;
use solana_pubkey::Pubkey;

/// Verifies init-stockpile can lazily create the stockpile SOL vault when it is absent.
#[test]
fn init_stockpile_instruction_marks_stockpile_sol_vault_writable() {
    let instruction =
        InstructionsHelper::init_stockpile_instruction(InitStockpileInstructionInputs {
            payer: Pubkey::new_unique(),
            stockpile_id: 3,
            zinc_mint: Pubkey::new_unique(),
            stockpile_token_account: PdaHelper::get_stockpile_token_account_address(),
            cluster_offset: 2026,
            computation_offset: 99,
        });

    assert_eq!(instruction.accounts.len(), 23);
    assert_eq!(
        instruction.accounts[17].pubkey,
        PdaHelper::get_stockpile_sol_vault_address()
    );
    assert!(instruction.accounts[17].is_writable);
    assert!(!instruction.accounts[17].is_signer);
}

/// Verifies init-stockpile can lazily create the stockpile token vault when it is absent.
#[test]
fn init_stockpile_instruction_marks_stockpile_token_vault_writable() {
    let stockpile_token_account = PdaHelper::get_stockpile_token_account_address();
    let instruction =
        InstructionsHelper::init_stockpile_instruction(InitStockpileInstructionInputs {
            payer: Pubkey::new_unique(),
            stockpile_id: 3,
            zinc_mint: Pubkey::new_unique(),
            stockpile_token_account,
            cluster_offset: 2026,
            computation_offset: 99,
        });

    assert_eq!(instruction.accounts[18].pubkey, stockpile_token_account);
    assert!(instruction.accounts[18].is_writable);
    assert!(!instruction.accounts[18].is_signer);
    assert_eq!(instruction.accounts[19].pubkey, PdaHelper::TOKEN_PROGRAM_ID);
    assert!(!instruction.accounts[19].is_writable);
    assert!(!instruction.accounts[19].is_signer);
    assert_eq!(
        instruction.accounts[22].pubkey,
        PdaHelper::ASSOCIATED_TOKEN_PROGRAM_ID
    );
    assert!(!instruction.accounts[22].is_writable);
    assert!(!instruction.accounts[22].is_signer);
}
