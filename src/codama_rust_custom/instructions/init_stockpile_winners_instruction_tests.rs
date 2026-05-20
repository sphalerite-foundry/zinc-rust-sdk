use super::{InitStockpileWinnersInstructionInputs, InstructionsHelper};
use crate::codama_rust::instructions::INIT_STOCKPILE_WINNERS_DISCRIMINATOR;
use crate::codama_rust_custom::pda::PdaHelper;
use solana_pubkey::Pubkey;

/// Verifies the helper derives every init-stockpile-winners account locally.
#[test]
fn init_stockpile_winners_instruction_derives_expected_accounts() {
    let payer = Pubkey::new_unique();
    let stockpile_id = 7;

    let instruction = InstructionsHelper::init_stockpile_winners_instruction(
        InitStockpileWinnersInstructionInputs {
            payer,
            stockpile_id,
        },
    );

    assert_eq!(instruction.accounts.len(), 6);
    assert_eq!(instruction.accounts[0].pubkey, payer);
    assert!(instruction.accounts[0].is_signer);
    assert!(instruction.accounts[0].is_writable);
    assert_eq!(
        instruction.accounts[1].pubkey,
        PdaHelper::get_config_address()
    );
    assert!(!instruction.accounts[1].is_writable);
    assert_eq!(
        instruction.accounts[2].pubkey,
        PdaHelper::get_board_address()
    );
    assert!(!instruction.accounts[2].is_writable);
    assert_eq!(
        instruction.accounts[3].pubkey,
        PdaHelper::get_stockpile_address(stockpile_id)
    );
    assert!(!instruction.accounts[3].is_writable);
    assert_eq!(
        instruction.accounts[4].pubkey,
        PdaHelper::get_stockpile_winners_address(stockpile_id)
    );
    assert!(instruction.accounts[4].is_writable);
    assert!(!instruction.accounts[4].is_signer);
    assert_eq!(
        instruction.accounts[5].pubkey,
        PdaHelper::get_system_program_address()
    );
    assert!(!instruction.accounts[5].is_writable);
    assert_eq!(instruction.data, INIT_STOCKPILE_WINNERS_DISCRIMINATOR);
}
