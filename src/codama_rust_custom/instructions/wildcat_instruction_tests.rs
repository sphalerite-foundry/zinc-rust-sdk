use super::{InstructionsHelper, SelectWildcatWinnerInstructionInputs};
use crate::codama_rust::ZINC_ID;
use crate::codama_rust_custom::pda::PdaHelper;
use solana_pubkey::Pubkey;

/// Verifies sidecar selection includes the concrete round Wildcat entries PDA.
#[test]
fn select_wildcat_winner_instruction_includes_sidecar_account() {
    let signer = Pubkey::new_unique();
    let round_id = 42;
    let instruction = InstructionsHelper::select_wildcat_winner_instruction(
        SelectWildcatWinnerInstructionInputs {
            signer,
            round_id,
            include_round_wildcat_entries: true,
        },
    );

    assert_eq!(instruction.accounts.len(), 4);
    assert_eq!(
        instruction.accounts[3].pubkey,
        PdaHelper::get_round_wildcat_entries_address(round_id)
    );
    assert!(!instruction.accounts[3].is_signer);
    assert!(instruction.accounts[3].is_writable);
}

/// Verifies legacy selection keeps Anchor's optional-account sentinel slot.
#[test]
fn select_wildcat_winner_instruction_includes_optional_account_sentinel() {
    let signer = Pubkey::new_unique();
    let instruction = InstructionsHelper::select_wildcat_winner_instruction(
        SelectWildcatWinnerInstructionInputs {
            signer,
            round_id: 42,
            include_round_wildcat_entries: false,
        },
    );

    assert_eq!(instruction.accounts.len(), 4);
    assert_eq!(instruction.accounts[3].pubkey, ZINC_ID);
    assert!(!instruction.accounts[3].is_signer);
    assert!(!instruction.accounts[3].is_writable);
}
