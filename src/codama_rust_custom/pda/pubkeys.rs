use crate::codama_rust::ZINC_ID;
use anchor_lang::prelude::Pubkey as AnchorPubkey;
use arcium_client::pda::{
    clock_acc, cluster_acc, comp_def_offset, computation_acc, computation_definition_acc,
    execpool_acc, fee_pool_acc, mempool_acc, mxe_acc, mxe_lut_acc, signer_acc,
};
use solana_pubkey::{pubkey, Pubkey};

/// Meteora account set needed to create and later identify the buyback pool.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct MeteoraPoolAccounts {
    /// Meteora pool authority PDA.
    pub pool_authority: Pubkey,
    /// Meteora static pool PDA.
    pub pool: Pubkey,
    /// Initial liquidity position PDA.
    pub position: Pubkey,
    /// Token-2022 account holding the initial position NFT.
    pub position_nft_account: Pubkey,
    /// Pool token A vault for ZINC.
    pub token_a_vault: Pubkey,
    /// Pool token B vault for WSOL.
    pub token_b_vault: Pubkey,
    /// Meteora event CPI authority PDA.
    pub event_authority: Pubkey,
}

pub struct PdaHelper;

/// Converts one Solana pubkey into the Anchor-compatible pubkey Arcium helpers expect.
fn to_anchor_pubkey(pubkey: Pubkey) -> AnchorPubkey {
    AnchorPubkey::new_from_array(pubkey.to_bytes())
}

/// Converts one Anchor-compatible pubkey into the Solana pubkey used by the Codama helpers.
fn to_solana_pubkey(pubkey: AnchorPubkey) -> Pubkey {
    Pubkey::new_from_array(pubkey.to_bytes())
}

impl PdaHelper {
    /// Meteora DAMM v2 constant product AMM program used by protocol buybacks.
    pub const METEORA_DAMM_V2_PROGRAM_ID: Pubkey =
        pubkey!("cpamdpZCGKUy5JxQXB4dcpGPiikHawvSWAd6mEn1sGG");
    /// Meteora DAMM v2 static config used by the canonical ZINC/WSOL buyback pool.
    pub const METEORA_STATIC_CONFIG_0002: Pubkey =
        pubkey!("FzvMYBQ29z2J21QPsABpJYYxQBEKGsxA6w6J2HYceFj8");
    /// Associated Token Program used to derive classic SPL token accounts.
    pub const ASSOCIATED_TOKEN_PROGRAM_ID: Pubkey =
        pubkey!("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL");
    /// SPL Token Program used by Zinc treasury token custody accounts.
    pub const TOKEN_PROGRAM_ID: Pubkey = pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");
    /// Token-2022 Program used by Meteora for position NFTs.
    pub const TOKEN_2022_PROGRAM_ID: Pubkey =
        pubkey!("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb");
    /// Wrapped SOL mint used as the buyback route input.
    pub const WSOL_MINT_ID: Pubkey = pubkey!("So11111111111111111111111111111111111111112");
    /// Canonical SPL USDC mint used by buyback SOL/USDC conversions.
    pub const USDC_MINT_ID: Pubkey = pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");
    pub const ARCIUM_PROGRAM_ID: Pubkey = pubkey!("Arcj82pX7HxYKLR92qvgZUAd7vGS1k4hQvAFcPATFdEQ");
    /// Metaplex Token Metadata Program that owns mint metadata PDAs.
    pub const METADATA_PROGRAM_ID: Pubkey = pubkey!("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");
    /// Rent sysvar account forwarded to Metaplex metadata creation.
    pub const RENT_SYSVAR_ID: Pubkey = pubkey!("SysvarRent111111111111111111111111111111111");
    /// Instructions sysvar forwarded to Meteora for instruction-stack validation.
    pub const INSTRUCTIONS_SYSVAR_ID: Pubkey =
        pubkey!("Sysvar1nstructions1111111111111111111111111");

    pub fn get_config_address() -> Pubkey {
        Pubkey::find_program_address(&[b"config"], &ZINC_ID).0
    }

    pub fn get_board_address() -> Pubkey {
        Pubkey::find_program_address(&[b"board"], &ZINC_ID).0
    }

    pub fn get_round_address(round_id: u64) -> Pubkey {
        Pubkey::find_program_address(&[b"round", round_id.to_le_bytes().as_ref()], &ZINC_ID).0
    }

    pub fn get_round_wildcat_entries_address(round_id: u64) -> Pubkey {
        Pubkey::find_program_address(
            &[b"round-wildcat", round_id.to_le_bytes().as_ref()],
            &ZINC_ID,
        )
        .0
    }

    pub fn get_round_secret_address(round_id: u64) -> Pubkey {
        Pubkey::find_program_address(
            &[b"round-secret", round_id.to_le_bytes().as_ref()],
            &ZINC_ID,
        )
        .0
    }

    pub fn get_stockpile_address(stockpile_id: u64) -> Pubkey {
        Pubkey::find_program_address(
            &[b"stockpile", stockpile_id.to_le_bytes().as_ref()],
            &ZINC_ID,
        )
        .0
    }

    pub fn get_stockpile_secret_address(stockpile_id: u64) -> Pubkey {
        Pubkey::find_program_address(
            &[b"stockpile-secret", stockpile_id.to_le_bytes().as_ref()],
            &ZINC_ID,
        )
        .0
    }

    /// Derives the ranked winners PDA for one stockpile cycle.
    pub fn get_stockpile_winners_address(stockpile_id: u64) -> Pubkey {
        Pubkey::find_program_address(
            &[b"stockpile-winners", stockpile_id.to_le_bytes().as_ref()],
            &ZINC_ID,
        )
        .0
    }

    pub fn get_stockpile_extras_address() -> Pubkey {
        Pubkey::find_program_address(&[b"stockpile-extras"], &ZINC_ID).0
    }

    pub fn get_miner_address(round_id: u64, player: &Pubkey) -> Pubkey {
        Pubkey::find_program_address(
            &[b"miner", round_id.to_le_bytes().as_ref(), player.as_ref()],
            &ZINC_ID,
        )
        .0
    }

    pub fn get_player_profile_address(player: &Pubkey) -> Pubkey {
        Pubkey::find_program_address(&[b"player-profile", player.as_ref()], &ZINC_ID).0
    }

    /// Derives one player-owned auto-miner session PDA.
    pub fn get_auto_miner_session_address(authority: &Pubkey) -> Pubkey {
        Pubkey::find_program_address(&[b"auto-miner-session", authority.as_ref()], &ZINC_ID).0
    }

    pub fn get_treasury_address() -> Pubkey {
        Pubkey::find_program_address(&[b"treasury"], &ZINC_ID).0
    }

    /// Derives the Metaplex metadata PDA attached to one mint.
    pub fn get_mint_metadata_address(mint: &Pubkey) -> Pubkey {
        Pubkey::find_program_address(
            &[
                b"metadata",
                Self::METADATA_PROGRAM_ID.as_ref(),
                mint.as_ref(),
            ],
            &Self::METADATA_PROGRAM_ID,
        )
        .0
    }

    /// Derives the treasury-owned Bonanza ZINC vault PDA created during config bootstrap.
    pub fn get_bonanza_token_account_address() -> Pubkey {
        Pubkey::find_program_address(&[b"treasury", b"bonanza-token-account"], &ZINC_ID).0
    }

    /// Derives the treasury-owned singleton vault for credited round ZINC rewards.
    pub fn get_round_zinc_reward_token_account_address() -> Pubkey {
        Pubkey::find_program_address(&[b"treasury", b"round-zinc-reward-token-account"], &ZINC_ID).0
    }

    /// Derives the round-owned ZINC payout vault PDA for one round.
    pub fn get_round_zinc_payout_token_account_address(
        round_id: u64,
        treasury: &Pubkey,
        zinc_mint: &Pubkey,
    ) -> Pubkey {
        Pubkey::find_program_address(
            &[
                b"zinc-payout-token-account",
                round_id.to_le_bytes().as_ref(),
                treasury.as_ref(),
                zinc_mint.as_ref(),
            ],
            &ZINC_ID,
        )
        .0
    }

    /// Derives the stockpile ZINC vault PDA created during config bootstrap.
    pub fn get_stockpile_token_account_address() -> Pubkey {
        Pubkey::find_program_address(&[b"treasury", b"stockpile-token-account"], &ZINC_ID).0
    }

    /// Derives the staking ZINC vault PDA created during config bootstrap.
    pub fn get_staking_token_account_address() -> Pubkey {
        Pubkey::find_program_address(&[b"treasury", b"staking-token-account"], &ZINC_ID).0
    }

    /// Derives the staking reward vault PDA created during config bootstrap.
    pub fn get_staking_reward_token_account_address() -> Pubkey {
        Pubkey::find_program_address(&[b"treasury", b"staking-reward-token-account"], &ZINC_ID).0
    }

    /// Derives the protocol-owned ZINC account used during buyback settlement.
    pub fn get_buyback_zinc_token_account_address() -> Pubkey {
        Pubkey::find_program_address(&[b"treasury", b"buyback-token-account"], &ZINC_ID).0
    }

    /// Derives the treasury-owned ZINC account that receives claimed buyback LP fees.
    pub fn get_buyback_fee_zinc_token_account_address() -> Pubkey {
        Pubkey::find_program_address(&[b"treasury", b"buyback-fee-zinc-token-account"], &ZINC_ID).0
    }

    /// Derives the treasury-owned WSOL account that receives claimed buyback LP fees.
    pub fn get_buyback_fee_wsol_token_account_address() -> Pubkey {
        Pubkey::find_program_address(&[b"treasury", b"buyback-fee-wsol-token-account"], &ZINC_ID).0
    }

    /// Derives the treasury-owned ZINC account that receives removed buyback LP principal.
    pub fn get_buyback_lp_zinc_token_account_address() -> Pubkey {
        Pubkey::find_program_address(&[b"treasury", b"buyback-lp-zinc-token-account"], &ZINC_ID).0
    }

    /// Derives the treasury-owned WSOL account that receives removed buyback LP principal.
    pub fn get_buyback_lp_wsol_token_account_address() -> Pubkey {
        Pubkey::find_program_address(&[b"treasury", b"buyback-lp-wsol-token-account"], &ZINC_ID).0
    }

    /// Derives a classic Associated Token Account address for one owner and mint.
    pub fn get_classic_ata(owner: &Pubkey, mint: &Pubkey) -> Pubkey {
        Pubkey::find_program_address(
            &[
                owner.as_ref(),
                Self::TOKEN_PROGRAM_ID.as_ref(),
                mint.as_ref(),
            ],
            &Self::ASSOCIATED_TOKEN_PROGRAM_ID,
        )
        .0
    }

    /// Derives the treasury-owned WSOL ATA used by buyback swaps.
    pub fn get_treasury_wsol_token_account_address() -> Pubkey {
        Self::get_classic_ata(&Self::get_treasury_address(), &Self::WSOL_MINT_ID)
    }

    /// Derives the treasury-owned USDC ATA used by buyback SOL/USDC conversions.
    pub fn get_treasury_usdc_token_account_address() -> Pubkey {
        Self::get_classic_ata(&Self::get_treasury_address(), &Self::USDC_MINT_ID)
    }

    /// Derives the temporary WSOL account used while converting buyback USDC back to SOL.
    pub fn get_buyback_usdc_to_sol_temporary_wsol_token_account_address() -> Pubkey {
        Pubkey::find_program_address(&[b"treasury", b"buyback-usdc-sol-wsol-temp"], &ZINC_ID).0
    }

    /// Derives the singleton stockpile SOL vault PDA.
    pub fn get_stockpile_sol_vault_address() -> Pubkey {
        Pubkey::find_program_address(&[b"stockpile-sol-vault"], &ZINC_ID).0
    }

    /// Derives the singleton buyback SOL vault PDA.
    pub fn get_buyback_sol_vault_address() -> Pubkey {
        Pubkey::find_program_address(&[b"buyback-sol-vault"], &ZINC_ID).0
    }

    /// Derives the singleton Meteora buyback pool manifest PDA.
    pub fn get_buyback_pool_address() -> Pubkey {
        Pubkey::find_program_address(&[b"buyback-pool"], &ZINC_ID).0
    }

    /// Derives every Meteora account needed for the canonical ZINC/WSOL buyback pool.
    pub fn get_meteora_buyback_pool_accounts(
        damm_config: &Pubkey,
        zinc_mint: &Pubkey,
        position_nft_mint: &Pubkey,
    ) -> MeteoraPoolAccounts {
        let (max_mint_seed, min_mint_seed) =
            Self::ordered_mint_seed_pair(&Self::WSOL_MINT_ID, zinc_mint);
        let pool_authority = Self::get_meteora_pool_authority_address();
        let pool = Self::derive_meteora_pda(&[
            b"pool",
            damm_config.as_ref(),
            max_mint_seed.as_ref(),
            min_mint_seed.as_ref(),
        ]);
        MeteoraPoolAccounts {
            pool_authority,
            pool,
            position: Self::derive_meteora_pda(&[b"position", position_nft_mint.as_ref()]),
            position_nft_account: Self::derive_meteora_pda(&[
                b"position_nft_account",
                position_nft_mint.as_ref(),
            ]),
            token_a_vault: Self::get_meteora_token_vault_address(zinc_mint, &pool),
            token_b_vault: Self::get_meteora_token_vault_address(&Self::WSOL_MINT_ID, &pool),
            event_authority: Self::get_meteora_event_authority_address(),
        }
    }

    /// Derives the global Meteora DAMM v2 pool authority PDA.
    pub fn get_meteora_pool_authority_address() -> Pubkey {
        Self::derive_meteora_pda(&[b"pool_authority"])
    }

    /// Derives the Meteora DAMM v2 event CPI authority PDA.
    pub fn get_meteora_event_authority_address() -> Pubkey {
        Self::derive_meteora_pda(&[b"__event_authority"])
    }

    /// Derives one Meteora DAMM v2 token vault PDA for a mint/pool pair.
    pub fn get_meteora_token_vault_address(mint: &Pubkey, pool: &Pubkey) -> Pubkey {
        Self::derive_meteora_pda(&[b"token_vault", mint.as_ref(), pool.as_ref()])
    }

    /// Sorts two mint keys the same way Meteora derives static pool PDAs.
    fn ordered_mint_seed_pair(left: &Pubkey, right: &Pubkey) -> ([u8; 32], [u8; 32]) {
        let left_bytes = left.to_bytes();
        let right_bytes = right.to_bytes();
        if left_bytes > right_bytes {
            (left_bytes, right_bytes)
        } else {
            (right_bytes, left_bytes)
        }
    }

    /// Derives one PDA owned by the Meteora DAMM v2 program.
    fn derive_meteora_pda(seeds: &[&[u8]]) -> Pubkey {
        Pubkey::find_program_address(seeds, &Self::METEORA_DAMM_V2_PROGRAM_ID).0
    }

    pub fn get_system_program_address() -> Pubkey {
        Pubkey::new_from_array(solana_system_interface::program::ID.to_bytes())
    }

    pub fn get_sign_pda_account_address() -> Pubkey {
        to_solana_pubkey(signer_acc(&to_anchor_pubkey(ZINC_ID)))
    }

    pub fn get_mxe_account_address() -> Pubkey {
        to_solana_pubkey(mxe_acc(&to_anchor_pubkey(ZINC_ID)))
    }

    pub fn get_program_lookup_table_address(lut_offset_slot: u64) -> Pubkey {
        to_solana_pubkey(mxe_lut_acc(&to_anchor_pubkey(ZINC_ID), lut_offset_slot))
    }

    pub fn get_mempool_account_address(cluster_offset: u32) -> Pubkey {
        to_solana_pubkey(mempool_acc(cluster_offset))
    }

    pub fn get_executing_pool_address(cluster_offset: u32) -> Pubkey {
        to_solana_pubkey(execpool_acc(cluster_offset))
    }

    pub fn get_computation_account_address(cluster_offset: u32, computation_offset: u64) -> Pubkey {
        to_solana_pubkey(computation_acc(cluster_offset, computation_offset))
    }

    pub fn get_comp_def_account_address(computation_definition_offset: u32) -> Pubkey {
        to_solana_pubkey(computation_definition_acc(
            &to_anchor_pubkey(ZINC_ID),
            computation_definition_offset,
        ))
    }

    pub fn get_init_round_rand_comp_def_account_address() -> Pubkey {
        Self::get_comp_def_account_address(comp_def_offset("init_round_rand"))
    }

    pub fn get_reveal_round_rand_comp_def_account_address() -> Pubkey {
        Self::get_comp_def_account_address(comp_def_offset("reveal_round_rand"))
    }

    pub fn get_init_stockpile_rand_comp_def_account_address() -> Pubkey {
        Self::get_comp_def_account_address(comp_def_offset("init_stockpile_rand"))
    }

    pub fn get_reveal_stockpile_rand_comp_def_account_address() -> Pubkey {
        Self::get_comp_def_account_address(comp_def_offset("reveal_stockpile_rand"))
    }

    pub fn get_settle_winning_stakes_batch_comp_def_account_address() -> Pubkey {
        Self::get_comp_def_account_address(comp_def_offset("settle_winning_stakes_batch"))
    }

    pub fn get_cluster_account_address(cluster_offset: u32) -> Pubkey {
        to_solana_pubkey(cluster_acc(cluster_offset))
    }

    pub fn get_pool_account_address() -> Pubkey {
        to_solana_pubkey(fee_pool_acc())
    }

    pub fn get_clock_account_address() -> Pubkey {
        to_solana_pubkey(clock_acc())
    }

    pub fn get_lut_program_address() -> Pubkey {
        Pubkey::new_from_array(solana_address_lookup_table_interface::program::ID.to_bytes())
    }
}
