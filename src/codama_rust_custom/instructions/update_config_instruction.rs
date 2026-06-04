use crate::codama_rust::instructions::{UpdateConfig, UpdateConfigInstructionArgs};
use crate::codama_rust::types::{RoundRandomnessMode, SettlementCapability};
use crate::codama_rust_custom::instructions::InstructionsHelper;
use crate::codama_rust_custom::pda::PdaHelper;
use solana_instruction::Instruction;
use solana_pubkey::Pubkey;

pub struct UpdateConfigInstructionInputs {
    /// Admin signer that is allowed to update the config.
    pub admin: Pubkey,
    /// Optional total deploy-time SOL fee skim in basis points.
    pub deploy_total_fee_bps: Option<u64>,
    /// Optional gross deploy-time SOL admin fee in basis points.
    pub deploy_admin_fee_bps: Option<u64>,
    /// Optional gross deploy-time SOL stockpile fee in basis points.
    pub deploy_stockpile_fee_bps: Option<u64>,
    /// Optional gross deploy-time SOL affiliate fee in basis points.
    pub deploy_affiliate_fee_bps: Option<u64>,
    /// Optional flat deploy-time stockpile-bricks bonus for referred players, in `x10k` units.
    pub deploy_affiliate_bonus_bricks_x10k: Option<u64>,
    /// Optional deterministic Wildcat cadence; `0` disables Wildcat.
    pub wildcat_round_frequency: Option<u64>,
    /// Optional direct-winner ZINC share reserved for the Wildcat winner, in ppm.
    pub wildcat_winner_zinc_share_ppm: Option<u64>,
    /// Optional max Wildcat candidate ranges snapshotted into new sidecar PDAs.
    pub wildcat_entry_capacity: Option<u32>,
    /// Optional toggle for writing new Wildcat candidate ranges to sidecar PDAs.
    pub wildcat_sidecar_enabled: Option<bool>,
    /// Optional first round id that must use sidecar storage when sidecar mode is enabled.
    pub wildcat_sidecar_activation_round_id: Option<u64>,
    /// Optional Bonanza roll modulo divisor; `1` makes every winner-positive round eligible.
    pub bonanza_hit_divisor: Option<u64>,
    /// Optional ZINC fee skim for round winner claims, in basis points.
    pub round_claim_zinc_fee_bps: Option<u64>,
    /// Optional minimum ZINC fee required to enter one stockpile cycle, in mint base units.
    pub stockpile_entry_min_zinc_fee: Option<u64>,
    /// Optional Stockpile bricks required per whole ZINC, in `x10k` units.
    pub stockpile_bricks_per_zinc_x10k: Option<u64>,
    /// Optional stockpile entry fee as a share of the live stockpile ZINC pot, in basis points.
    pub stockpile_entry_pot_fee_bps: Option<u64>,
    /// Optional accepted-entry step multiplier for stockpile entry costs, in basis points.
    pub stockpile_entry_step_bps: Option<u64>,
    /// Optional minimum refill size as a share of current stockpile entry bricks.
    pub stockpile_refill_min_entry_bps: Option<u64>,
    /// Optional staking brick issuance rate per claimed ZINC, in `x10k` units.
    pub staking_bricks_per_zinc_x10k: Option<u64>,
    /// Optional slots over which newly melted staking rewards vest.
    pub staking_reward_vesting_slots: Option<u64>,
    /// Optional Arcium priority price per CU for round reveals, in micro-lamports.
    pub arcium_reveal_cu_price_micro: Option<u64>,
    /// Optional toggle for affiliate withdrawals.
    pub affiliate_withdrawals_enabled: Option<bool>,
    /// Optional round duration in slots.
    pub round_duration_slots: Option<u64>,
    /// Optional stockpile duration in slots.
    pub stockpile_duration_slots: Option<u64>,
    /// Optional inter-round delay in slots after winning tile reveal.
    pub round_start_delay_slots: Option<u64>,
    /// Optional minimum stockpile entry size in `x10k` bricks.
    pub stockpile_min_entry_bricks_x10k: Option<u64>,
    /// Optional curve-minted ZINC admin skim in basis points.
    pub curve_admin_fee_bps: Option<u64>,
    /// Optional share of post-skim ZINC allocated to direct winners.
    pub winner_zinc_share_bps: Option<u64>,
    /// Optional share of post-skim ZINC allocated to stockpile ZINC.
    pub stockpile_zinc_share_bps: Option<u64>,
    /// Optional no-winner direct-winner ZINC redirect share allocated to Bonanza.
    pub no_winner_direct_winner_zinc_bonanza_share_bps: Option<u64>,
    /// Optional no-winner direct-winner ZINC redirect share allocated to stockpile.
    pub no_winner_direct_winner_zinc_stockpile_share_bps: Option<u64>,
    /// Optional minimum gross lamports required to deploy into a round.
    pub min_deploy_lamports: Option<u64>,
    /// Optional launch-time maximum mint for one round before ZINC factoring and support caps.
    pub curve_max_round_mint: Option<u64>,
    /// Optional SOL factoring lamports for the deploy curve.
    pub curve_saturation_lamports: Option<u64>,
    /// Optional ZINC factoring amount that halves scarcity emissions.
    pub curve_history_minted: Option<u64>,
    /// Optional min target ZINC support lamports required for one emitted token.
    pub curve_target_support_lamports_per_zinc: Option<u64>,
    /// Optional maximum curve ZINC supply.
    pub curve_max_supply: Option<u64>,
    /// Optional crank authority to persist on config and board.
    pub crank: Option<Pubkey>,
    /// Optional live randomness reveal path for closed rounds.
    pub round_randomness_mode: Option<RoundRandomnessMode>,
    /// Optional live settlement capability selected by the operator.
    pub settlement_capability: Option<SettlementCapability>,
    /// Optional slot delay used before sampling SlotHashes in blockhash mode.
    pub blockhash_reveal_delay_slots: Option<u64>,
    /// Optional number of ranked Stockpile winners to pay per cycle.
    pub stockpile_winner_count: Option<u8>,
    /// Optional ranked Stockpile winner share schedule in basis points.
    pub stockpile_winner_share_bps: Option<[u64; 10]>,
    /// Optional server BabyJub public key X field element bytes for ZK mask attestations.
    pub zk_mask_server_babyjub_pubkey_x: Option<[u8; 32]>,
    /// Optional server BabyJub public key Y field element bytes for ZK mask attestations.
    pub zk_mask_server_babyjub_pubkey_y: Option<[u8; 32]>,
    /// Optional accepted server key version for ZK mask attestations.
    pub zk_mask_server_key_version: Option<u64>,
    /// Optional accepted circuit version for ZK mask attestations.
    pub zk_mask_circuit_version: Option<u64>,
    /// Optional emergency switch for skipping init-round Arcium CPI.
    pub skip_arcium_init_cpi: Option<bool>,
}

impl InstructionsHelper {
    pub fn update_config_instruction(inputs: UpdateConfigInstructionInputs) -> Instruction {
        let UpdateConfigInstructionInputs {
            admin,
            deploy_total_fee_bps,
            deploy_admin_fee_bps,
            deploy_stockpile_fee_bps,
            deploy_affiliate_fee_bps,
            deploy_affiliate_bonus_bricks_x10k,
            wildcat_round_frequency,
            wildcat_winner_zinc_share_ppm,
            wildcat_entry_capacity,
            wildcat_sidecar_enabled,
            wildcat_sidecar_activation_round_id,
            bonanza_hit_divisor,
            round_claim_zinc_fee_bps,
            stockpile_entry_min_zinc_fee,
            stockpile_bricks_per_zinc_x10k,
            stockpile_entry_pot_fee_bps,
            stockpile_entry_step_bps,
            stockpile_refill_min_entry_bps,
            staking_bricks_per_zinc_x10k,
            staking_reward_vesting_slots,
            arcium_reveal_cu_price_micro,
            affiliate_withdrawals_enabled,
            round_duration_slots,
            stockpile_duration_slots,
            round_start_delay_slots,
            stockpile_min_entry_bricks_x10k,
            curve_admin_fee_bps,
            winner_zinc_share_bps,
            stockpile_zinc_share_bps,
            no_winner_direct_winner_zinc_bonanza_share_bps,
            no_winner_direct_winner_zinc_stockpile_share_bps,
            min_deploy_lamports,
            curve_max_round_mint,
            curve_saturation_lamports,
            curve_history_minted,
            curve_target_support_lamports_per_zinc,
            curve_max_supply,
            crank,
            round_randomness_mode,
            settlement_capability,
            blockhash_reveal_delay_slots,
            stockpile_winner_count,
            stockpile_winner_share_bps,
            zk_mask_server_babyjub_pubkey_x,
            zk_mask_server_babyjub_pubkey_y,
            zk_mask_server_key_version,
            zk_mask_circuit_version,
            skip_arcium_init_cpi,
        } = inputs;
        UpdateConfig {
            admin,
            config: PdaHelper::get_config_address(),
            board: PdaHelper::get_board_address(),
        }
        .instruction(UpdateConfigInstructionArgs {
            deploy_total_fee_bps,
            affiliate_withdrawals_enabled,
            round_duration_slots,
            stockpile_duration_slots,
            round_start_delay_slots,
            stockpile_min_entry_bricks_x10k,
            deploy_admin_fee_bps,
            deploy_stockpile_fee_bps,
            deploy_affiliate_fee_bps,
            curve_admin_fee_bps,
            winner_zinc_share_bps,
            stockpile_zinc_share_bps,
            no_winner_direct_winner_zinc_bonanza_share_bps,
            no_winner_direct_winner_zinc_stockpile_share_bps,
            min_deploy_lamports,
            curve_max_round_mint,
            curve_saturation_lamports,
            curve_history_minted,
            curve_target_support_lamports_per_zinc,
            curve_max_supply,
            crank,
            deploy_affiliate_bonus_bricks_x10k,
            wildcat_round_frequency,
            wildcat_winner_zinc_share_ppm,
            wildcat_entry_capacity,
            wildcat_sidecar_enabled,
            wildcat_sidecar_activation_round_id,
            bonanza_hit_divisor,
            round_claim_zinc_fee_bps,
            stockpile_entry_min_zinc_fee,
            stockpile_bricks_per_zinc_x10k,
            stockpile_entry_pot_fee_bps,
            stockpile_entry_step_bps,
            stockpile_refill_min_entry_bps,
            staking_bricks_per_zinc_x10k,
            staking_reward_vesting_slots,
            arcium_reveal_cu_price_micro,
            round_randomness_mode,
            settlement_capability,
            blockhash_reveal_delay_slots,
            stockpile_winner_count,
            stockpile_winner_share_bps,
            zk_mask_server_babyjub_pubkey_x,
            zk_mask_server_babyjub_pubkey_y,
            zk_mask_server_key_version,
            zk_mask_circuit_version,
            skip_arcium_init_cpi,
        })
    }
}
