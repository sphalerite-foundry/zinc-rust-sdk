use super::{CloseRoundInstructionInputs, InstructionsHelper};
use crate::codama_rust_custom::pda::PdaHelper;
use solana_pubkey::Pubkey;

/// Verifies close-round mints to dedicated liquidity ZINC custody in the generated account order.
#[test]
fn close_round_instruction_includes_liquidity_zinc_custody_account() {
    let bonanza_token_account = Pubkey::new_unique();
    let stockpile_token_account = Pubkey::new_unique();
    let instruction = InstructionsHelper::close_round_instruction(CloseRoundInstructionInputs {
        signer: Pubkey::new_unique(),
        round_id: 44,
        stockpile_id: Some(7),
        zinc_mint: Pubkey::new_unique(),
        curve_admin_token_account: Pubkey::new_unique(),
        bonanza_token_account,
        stockpile_token_account,
    });

    assert_eq!(instruction.accounts[7].pubkey, bonanza_token_account);
    assert_eq!(
        instruction.accounts[8].pubkey,
        PdaHelper::get_liquidity_zinc_token_account_address()
    );
    assert_eq!(instruction.accounts[10].pubkey, stockpile_token_account);
    assert!(instruction.accounts[7].is_writable);
    assert!(instruction.accounts[8].is_writable);
    assert!(instruction.accounts[10].is_writable);
}
