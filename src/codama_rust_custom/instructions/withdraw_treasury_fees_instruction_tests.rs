use super::{InstructionsHelper, WithdrawTreasuryFeesInstructionInputs};
use crate::codama_rust::instructions::WITHDRAW_TREASURY_FEES_DISCRIMINATOR;
use crate::codama_rust_custom::pda::PdaHelper;
use solana_pubkey::Pubkey;

/// Reads one encoded little-endian `u64` field from instruction data.
fn read_data_u64(data: &[u8], offset: usize) -> u64 {
    let mut bytes = [0_u8; 8];
    bytes.copy_from_slice(&data[offset..offset + 8]);
    u64::from_le_bytes(bytes)
}

/// Verifies the wrapper encodes exact SOL and ZINC amounts instead of sweep-all semantics.
#[test]
fn withdraw_treasury_fees_instruction_encodes_exact_amounts() {
    let zinc_mint = Pubkey::new_unique();
    let admin_token_account = Pubkey::new_unique();
    let instruction = InstructionsHelper::withdraw_treasury_fees_instruction(
        WithdrawTreasuryFeesInstructionInputs {
            admin: Pubkey::new_unique(),
            zinc_mint,
            admin_token_account,
            sol_lamports: 12_345,
            zinc_amount: 67_890,
        },
    );

    assert_eq!(
        &instruction.data[..8],
        WITHDRAW_TREASURY_FEES_DISCRIMINATOR.as_slice()
    );
    assert_eq!(read_data_u64(&instruction.data, 8), 12_345);
    assert_eq!(read_data_u64(&instruction.data, 16), 67_890);
    assert_eq!(
        instruction.accounts[4].pubkey,
        PdaHelper::get_classic_ata(&PdaHelper::get_treasury_address(), &zinc_mint)
    );
    assert_eq!(instruction.accounts[5].pubkey, admin_token_account);
}
