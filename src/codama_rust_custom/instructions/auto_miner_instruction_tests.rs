use super::{DeployRoundFromAutoSessionInstructionInputs, InstructionsHelper};
use crate::codama_rust_custom::pda::PdaHelper;
use solana_pubkey::Pubkey;

/// Builds one auto-miner deploy instruction with stable placeholder ciphertext inputs.
fn auto_deploy_instruction() -> solana_instruction::Instruction {
    InstructionsHelper::deploy_round_from_auto_session_instruction(
        DeployRoundFromAutoSessionInstructionInputs {
            executor: Pubkey::new_unique(),
            authority: Pubkey::new_unique(),
            stockpile_id: Some(1),
            round_id: 1,
            affiliate: Some(Pubkey::new_unique()),
            mask_encryption_key: [0; 32],
            mask_nonce: 0,
            mask_ciphertext: [0; 64],
            zk_mask_attestation: None,
        },
    )
}

/// Verifies auto-miner deploys use the same dedicated SOL custody metas as manual deploys.
#[test]
fn auto_deploy_instruction_includes_dedicated_sol_custody_accounts() {
    let instruction = auto_deploy_instruction();

    assert_eq!(
        instruction.accounts[9].pubkey,
        PdaHelper::get_stockpile_sol_vault_address()
    );
    assert_eq!(
        instruction.accounts[10].pubkey,
        PdaHelper::get_bonanza_sol_vault_address()
    );
    assert_eq!(
        instruction.accounts[11].pubkey,
        PdaHelper::get_buyback_sol_vault_address()
    );
    assert!(instruction.accounts[9].is_writable);
    assert!(instruction.accounts[10].is_writable);
    assert!(instruction.accounts[11].is_writable);
}
