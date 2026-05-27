use super::{
    BuybackInstructionInputs, ClaimBuybackPoolFeesInstructionInputs,
    CreateBuybackPoolInstructionInputs, InstructionsHelper, LockBuybackLiquidityInstructionInputs,
    RemoveBuybackLiquidityInstructionInputs,
};
use crate::codama_rust_custom::pda::PdaHelper;
use solana_pubkey::Pubkey;

/// Verifies that treasury WSOL sync targets the token account used by buybacks.
#[test]
fn sync_treasury_wsol_instruction_targets_treasury_wsol() {
    let instruction = InstructionsHelper::sync_treasury_wsol_instruction();

    assert_eq!(
        instruction.program_id,
        crate::codama_rust_custom::pda::PdaHelper::TOKEN_PROGRAM_ID
    );
    assert_eq!(instruction.data, vec![17]);
    assert_eq!(instruction.accounts.len(), 1);
    assert_eq!(
        instruction.accounts[0].pubkey,
        crate::codama_rust_custom::pda::PdaHelper::get_treasury_wsol_token_account_address()
    );
    assert!(instruction.accounts[0].is_writable);
    assert!(!instruction.accounts[0].is_signer);
}

/// Verifies direct Meteora buyback instructions carry the expected account order.
#[test]
fn buyback_instruction_uses_stored_pool_accounts() {
    let pool_authority = Pubkey::new_unique();
    let pool = Pubkey::new_unique();
    let token_a_vault = Pubkey::new_unique();
    let token_b_vault = Pubkey::new_unique();
    let event_authority = Pubkey::new_unique();

    let instruction = InstructionsHelper::buyback_instruction(BuybackInstructionInputs {
        signer: Pubkey::new_unique(),
        zinc_mint: Pubkey::new_unique(),
        amount_in: 42,
        min_zinc_out: 7,
        pool_authority,
        pool,
        token_a_vault,
        token_b_vault,
        event_authority,
    });

    assert_eq!(&instruction.data[..8], &[106, 117, 64, 30, 56, 69, 7, 45]);
    assert_eq!(instruction.accounts[9].pubkey, pool_authority);
    assert_eq!(instruction.accounts[10].pubkey, pool);
    assert_eq!(instruction.accounts[11].pubkey, token_a_vault);
    assert_eq!(instruction.accounts[12].pubkey, token_b_vault);
    assert_eq!(instruction.accounts[13].pubkey, event_authority);
    assert!(!instruction.accounts[13].is_writable);
}

/// Verifies Meteora pool initialization instructions derive the canonical ZINC/WSOL account set.
#[test]
fn create_buyback_pool_instruction_uses_canonical_meteora_accounts() {
    let admin = Pubkey::new_unique();
    let zinc_mint = Pubkey::new_unique();
    let damm_config = Pubkey::new_unique();
    let position_nft_mint = Pubkey::new_unique();
    let meteora_accounts =
        PdaHelper::get_meteora_buyback_pool_accounts(&damm_config, &zinc_mint, &position_nft_mint);

    let instruction =
        InstructionsHelper::create_buyback_pool_instruction(CreateBuybackPoolInstructionInputs {
            admin,
            zinc_mint,
            damm_config,
            position_nft_mint,
            lp_zinc_amount: 100,
            initial_wsol_lamports: 200,
            liquidity: 300,
            sqrt_price: 400,
            activation_point: Some(500),
        });

    assert_eq!(
        &instruction.data[..8],
        &[149, 112, 78, 105, 27, 160, 104, 167]
    );
    assert_eq!(instruction.accounts[0].pubkey, admin);
    assert!(instruction.accounts[0].is_signer);
    assert_eq!(instruction.accounts[4].pubkey, PdaHelper::WSOL_MINT_ID);
    assert_eq!(instruction.accounts[5].pubkey, zinc_mint);
    assert_eq!(
        instruction.accounts[6].pubkey,
        PdaHelper::get_classic_ata(&admin, &PdaHelper::WSOL_MINT_ID)
    );
    assert_eq!(
        instruction.accounts[7].pubkey,
        PdaHelper::get_classic_ata(&admin, &zinc_mint)
    );
    assert_eq!(instruction.accounts[8].pubkey, damm_config);
    assert_eq!(
        instruction.accounts[9].pubkey,
        PdaHelper::METEORA_DAMM_V2_PROGRAM_ID
    );
    assert_eq!(
        instruction.accounts[10].pubkey,
        meteora_accounts.pool_authority
    );
    assert_eq!(instruction.accounts[11].pubkey, meteora_accounts.pool);
    assert_eq!(instruction.accounts[12].pubkey, position_nft_mint);
    assert!(instruction.accounts[12].is_signer);
    assert_eq!(
        instruction.accounts[13].pubkey,
        meteora_accounts.position_nft_account
    );
    assert_eq!(instruction.accounts[14].pubkey, meteora_accounts.position);
    assert_eq!(
        instruction.accounts[15].pubkey,
        meteora_accounts.token_a_vault
    );
    assert_eq!(
        instruction.accounts[16].pubkey,
        meteora_accounts.token_b_vault
    );
    assert_eq!(
        instruction.accounts[17].pubkey,
        meteora_accounts.event_authority
    );
}

/// Verifies fee claims use dedicated treasury-controlled custody accounts.
#[test]
fn claim_buyback_pool_fees_instruction_uses_fee_custody_accounts() {
    let admin = Pubkey::new_unique();
    let zinc_mint = Pubkey::new_unique();
    let pool_authority = Pubkey::new_unique();
    let pool = Pubkey::new_unique();
    let position = Pubkey::new_unique();
    let position_nft_account = Pubkey::new_unique();
    let token_a_vault = Pubkey::new_unique();
    let token_b_vault = Pubkey::new_unique();
    let event_authority = Pubkey::new_unique();

    let instruction = InstructionsHelper::claim_buyback_pool_fees_instruction(
        ClaimBuybackPoolFeesInstructionInputs {
            admin,
            zinc_mint,
            pool_authority,
            pool,
            position,
            position_nft_account,
            token_a_vault,
            token_b_vault,
            event_authority,
        },
    );

    assert_eq!(
        &instruction.data[..8],
        &[124, 86, 221, 109, 203, 99, 227, 187]
    );
    assert_eq!(instruction.accounts[0].pubkey, admin);
    assert!(instruction.accounts[0].is_signer);
    assert_eq!(
        instruction.accounts[6].pubkey,
        PdaHelper::get_buyback_fee_zinc_token_account_address()
    );
    assert!(instruction.accounts[6].is_writable);
    assert_eq!(
        instruction.accounts[7].pubkey,
        PdaHelper::get_buyback_fee_wsol_token_account_address()
    );
    assert!(instruction.accounts[7].is_writable);
    assert_eq!(
        instruction.accounts[8].pubkey,
        PdaHelper::get_classic_ata(&admin, &zinc_mint)
    );
    assert!(instruction.accounts[8].is_writable);
    assert_eq!(
        instruction.accounts[9].pubkey,
        PdaHelper::get_classic_ata(&admin, &PdaHelper::WSOL_MINT_ID)
    );
    assert!(instruction.accounts[9].is_writable);
    assert_eq!(instruction.accounts[10].pubkey, pool_authority);
    assert_eq!(instruction.accounts[11].pubkey, pool);
    assert_eq!(instruction.accounts[12].pubkey, position);
    assert!(instruction.accounts[12].is_writable);
    assert_eq!(instruction.accounts[13].pubkey, position_nft_account);
    assert_eq!(instruction.accounts[14].pubkey, token_a_vault);
    assert_eq!(instruction.accounts[15].pubkey, token_b_vault);
    assert_eq!(instruction.accounts[16].pubkey, event_authority);
    assert_eq!(
        instruction.accounts[18].pubkey,
        PdaHelper::ASSOCIATED_TOKEN_PROGRAM_ID
    );
}

/// Verifies LP locking uses the stored protocol-owned Meteora position.
#[test]
fn lock_buyback_liquidity_instruction_uses_stored_position_accounts() {
    let admin = Pubkey::new_unique();
    let pool = Pubkey::new_unique();
    let position = Pubkey::new_unique();
    let position_nft_account = Pubkey::new_unique();
    let event_authority = Pubkey::new_unique();

    let instruction = InstructionsHelper::lock_buyback_liquidity_instruction(
        LockBuybackLiquidityInstructionInputs {
            admin,
            liquidity_delta: 42,
            pool,
            position,
            position_nft_account,
            event_authority,
        },
    );

    assert_eq!(
        &instruction.data[..8],
        &[232, 167, 111, 218, 205, 250, 130, 93]
    );
    assert_eq!(instruction.accounts[0].pubkey, admin);
    assert!(instruction.accounts[0].is_signer);
    assert_eq!(instruction.accounts[4].pubkey, pool);
    assert!(instruction.accounts[4].is_writable);
    assert_eq!(instruction.accounts[5].pubkey, position);
    assert!(instruction.accounts[5].is_writable);
    assert_eq!(instruction.accounts[6].pubkey, position_nft_account);
    assert!(!instruction.accounts[6].is_writable);
    assert_eq!(instruction.accounts[7].pubkey, event_authority);
    assert!(!instruction.accounts[7].is_writable);
    assert_eq!(
        instruction.accounts[8].pubkey,
        PdaHelper::METEORA_DAMM_V2_PROGRAM_ID
    );
}

/// Verifies LP removal uses dedicated principal custody accounts.
#[test]
fn remove_buyback_liquidity_instruction_uses_lp_custody_accounts() {
    let admin = Pubkey::new_unique();
    let zinc_mint = Pubkey::new_unique();
    let pool_authority = Pubkey::new_unique();
    let pool = Pubkey::new_unique();
    let position = Pubkey::new_unique();
    let position_nft_account = Pubkey::new_unique();
    let token_a_vault = Pubkey::new_unique();
    let token_b_vault = Pubkey::new_unique();
    let event_authority = Pubkey::new_unique();

    let instruction = InstructionsHelper::remove_buyback_liquidity_instruction(
        RemoveBuybackLiquidityInstructionInputs {
            admin,
            zinc_mint,
            liquidity_delta: 42,
            token_a_amount_threshold: 7,
            token_b_amount_threshold: 9,
            pool_authority,
            pool,
            position,
            position_nft_account,
            token_a_vault,
            token_b_vault,
            event_authority,
        },
    );

    assert_eq!(
        &instruction.data[..8],
        &[162, 33, 230, 119, 49, 191, 203, 163]
    );
    assert_eq!(instruction.accounts[0].pubkey, admin);
    assert!(instruction.accounts[0].is_signer);
    assert_eq!(instruction.accounts[5].pubkey, zinc_mint);
    assert_eq!(
        instruction.accounts[6].pubkey,
        PdaHelper::get_buyback_lp_zinc_token_account_address()
    );
    assert!(instruction.accounts[6].is_writable);
    assert_eq!(
        instruction.accounts[7].pubkey,
        PdaHelper::get_buyback_lp_wsol_token_account_address()
    );
    assert!(instruction.accounts[7].is_writable);
    assert_eq!(instruction.accounts[8].pubkey, pool_authority);
    assert_eq!(instruction.accounts[9].pubkey, pool);
    assert_eq!(instruction.accounts[10].pubkey, position);
    assert!(instruction.accounts[10].is_writable);
    assert_eq!(instruction.accounts[11].pubkey, position_nft_account);
    assert_eq!(instruction.accounts[12].pubkey, token_a_vault);
    assert_eq!(instruction.accounts[13].pubkey, token_b_vault);
    assert_eq!(instruction.accounts[14].pubkey, event_authority);
}
