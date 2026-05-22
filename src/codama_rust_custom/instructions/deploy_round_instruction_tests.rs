use super::{DeployRoundInstructionInputs, InstructionsHelper};
use solana_pubkey::Pubkey;

/// Builds one deploy instruction with stable placeholder ciphertext inputs.
fn deploy_instruction(
    signer: Pubkey,
    affiliate: Option<Pubkey>,
) -> solana_instruction::Instruction {
    InstructionsHelper::deploy_round_instruction(DeployRoundInstructionInputs {
        signer,
        round_id: 1,
        stockpile_id: Some(1),
        total_amount: 1,
        affiliate,
        mask_encryption_key: [0; 32],
        mask_nonce: 0,
        mask_ciphertext: [0; 64],
    })
}

/// Verifies self-referral inputs do not create duplicate player-profile account metas.
#[test]
fn deploy_round_instruction_omits_self_referral_accounts() {
    let signer = Pubkey::new_unique();
    let instruction = deploy_instruction(signer, Some(signer));

    assert_eq!(instruction.accounts[10].pubkey, crate::ZINC_ID);
    assert_eq!(instruction.accounts[11].pubkey, crate::ZINC_ID);
}

/// Verifies third-party affiliate inputs still include affiliate account metas.
#[test]
fn deploy_round_instruction_keeps_third_party_affiliate_accounts() {
    let signer = Pubkey::new_unique();
    let affiliate = Pubkey::new_unique();
    let instruction = deploy_instruction(signer, Some(affiliate));

    assert_eq!(instruction.accounts[10].pubkey, affiliate);
    assert_eq!(
        instruction.accounts[11].pubkey,
        crate::codama_rust_custom::pda::PdaHelper::get_player_profile_address(&affiliate)
    );
}
