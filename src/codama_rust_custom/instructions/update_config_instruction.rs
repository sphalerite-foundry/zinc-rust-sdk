use crate::codama_rust::instructions::{UpdateConfig, UpdateConfigInstructionArgs};
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
    /// Optional Bonanza roll modulo divisor; `1` makes every winner-positive round eligible.
    pub bonanza_hit_divisor: Option<u64>,
    /// Optional ZINC fee skim for round winner claims, in basis points.
    pub round_claim_zinc_fee_bps: Option<u64>,
    /// Optional minimum ZINC fee required to enter one stockpile cycle, in mint base units.
    pub stockpile_entry_min_zinc_fee: Option<u64>,
    /// Optional stockpile entry fee as a share of the live stockpile ZINC pot, in basis points.
    pub stockpile_entry_pot_fee_bps: Option<u64>,
    /// Optional accepted-entry step multiplier for stockpile entry costs, in basis points.
    pub stockpile_entry_step_bps: Option<u64>,
    /// Optional staking brick issuance rate per claimed ZINC, in `x10k` units.
    pub staking_bricks_per_zinc_x10k: Option<u64>,
    /// Optional toggle for affiliate withdrawals.
    pub affiliate_withdrawals_enabled: Option<bool>,
    /// Optional round duration in slots.
    pub round_duration_slots: Option<u64>,
    /// Optional stockpile duration in slots.
    pub stockpile_duration_slots: Option<u64>,
    /// Optional minimum stockpile entry size in `x10k` bricks.
    pub stockpile_min_entry_bricks_x10k: Option<u64>,
    /// Optional curve-minted ZINC admin skim in basis points.
    pub curve_admin_fee_bps: Option<u64>,
    /// Optional share of post-skim ZINC allocated to direct winners.
    pub winner_zinc_share_bps: Option<u64>,
    /// Optional share of post-skim ZINC allocated to stockpile ZINC.
    pub stockpile_zinc_share_bps: Option<u64>,
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
            bonanza_hit_divisor,
            round_claim_zinc_fee_bps,
            stockpile_entry_min_zinc_fee,
            stockpile_entry_pot_fee_bps,
            stockpile_entry_step_bps,
            staking_bricks_per_zinc_x10k,
            affiliate_withdrawals_enabled,
            round_duration_slots,
            stockpile_duration_slots,
            stockpile_min_entry_bricks_x10k,
            curve_admin_fee_bps,
            winner_zinc_share_bps,
            stockpile_zinc_share_bps,
            min_deploy_lamports,
            curve_max_round_mint,
            curve_saturation_lamports,
            curve_history_minted,
            curve_target_support_lamports_per_zinc,
            curve_max_supply,
            crank,
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
            stockpile_min_entry_bricks_x10k,
            deploy_admin_fee_bps,
            deploy_stockpile_fee_bps,
            deploy_affiliate_fee_bps,
            curve_admin_fee_bps,
            winner_zinc_share_bps,
            stockpile_zinc_share_bps,
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
            bonanza_hit_divisor,
            round_claim_zinc_fee_bps,
            stockpile_entry_min_zinc_fee,
            stockpile_entry_pot_fee_bps,
            stockpile_entry_step_bps,
            staking_bricks_per_zinc_x10k,
        })
    }
}
