use super::{FinalizeNoWinnerRoundInstructionInputs, InstructionsHelper};
use solana_pubkey::Pubkey;

/// Verifies no-winner finalization routes Bonanza gross ZINC through Bonanza and Liquidity metas.
#[test]
fn finalize_no_winner_instruction_includes_liquidity_zinc_custody_account() {
    let bonanza_token_account = Pubkey::new_unique();
    let liquidity_zinc_token_account = Pubkey::new_unique();
    let stockpile_token_account = Pubkey::new_unique();
    let instruction = InstructionsHelper::finalize_no_winner_round_instruction(
        FinalizeNoWinnerRoundInstructionInputs {
            signer: Pubkey::new_unique(),
            round_id: 44,
            zinc_mint: Pubkey::new_unique(),
            bonanza_token_account,
            liquidity_zinc_token_account,
            stockpile_token_account,
        },
    );

    assert_eq!(instruction.accounts.len(), 12);
    assert_eq!(instruction.accounts[8].pubkey, bonanza_token_account);
    assert_eq!(instruction.accounts[9].pubkey, liquidity_zinc_token_account);
    assert_eq!(instruction.accounts[10].pubkey, stockpile_token_account);
    assert!(instruction.accounts[8].is_writable);
    assert!(instruction.accounts[9].is_writable);
    assert!(instruction.accounts[10].is_writable);
}
