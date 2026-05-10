use crate::codama_rust::instructions::{
    ClaimPlayerSolRewards, ClaimPlayerZincRewards, ClaimRoundSol, CreditRoundRewards,
};
use crate::codama_rust_custom::instructions::InstructionsHelper;
use crate::codama_rust_custom::pda::PdaHelper;
use solana_instruction::Instruction;
use solana_pubkey::pubkey;
use solana_pubkey::Pubkey;

const ASSOCIATED_TOKEN_PROGRAM_ID: solana_pubkey::Pubkey =
    pubkey!("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL");
const TOKEN_PROGRAM_ID: solana_pubkey::Pubkey =
    pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");

pub struct ClaimPlayerZincRewardsInstructionInputs {
    /// Player signer submitting the aggregate round ZINC reward claim transaction.
    pub signer: Pubkey,
    /// Protocol ZINC mint persisted on the treasury account.
    pub zinc_mint: Pubkey,
}

/// Inputs used to claim aggregate round SOL rewards from a player profile.
pub struct ClaimPlayerSolRewardsInstructionInputs {
    /// Player signer that owns the profile and receives the SOL payout.
    pub signer: Pubkey,
}

/// Inputs used to credit one settled round's rewards into a player profile.
pub struct CreditRoundRewardsInstructionInputs {
    /// Crank signer submitting the automatic reward credit transaction.
    pub signer: Pubkey,
    /// Player whose miner rewards are credited.
    pub player: Pubkey,
    /// Settled round being credited.
    pub round_id: u64,
    /// Protocol ZINC mint persisted on the treasury account.
    pub zinc_mint: Pubkey,
}

/// Inputs used to build an automatic SOL claim-round instruction.
pub struct ClaimRoundSolInstructionInputs {
    /// Crank signer submitting the automatic SOL claim transaction.
    pub signer: Pubkey,
    /// SOL recipient whose miner PDA is derived.
    pub player: Pubkey,
    /// Settled round being claimed.
    pub round_id: u64,
}

/// Derives the canonical player ATA used by the ZINC claim instruction.
fn get_player_zinc_token_account(player: &Pubkey, zinc_mint: &Pubkey) -> Pubkey {
    Pubkey::find_program_address(
        &[
            player.as_ref(),
            TOKEN_PROGRAM_ID.as_ref(),
            zinc_mint.as_ref(),
        ],
        &ASSOCIATED_TOKEN_PROGRAM_ID,
    )
    .0
}

impl InstructionsHelper {
    /// Builds the crank-only round reward credit instruction.
    pub fn credit_round_rewards_instruction(
        inputs: CreditRoundRewardsInstructionInputs,
    ) -> Instruction {
        let CreditRoundRewardsInstructionInputs {
            signer,
            player,
            round_id,
            zinc_mint,
        } = inputs;
        let treasury = PdaHelper::get_treasury_address();
        let round_zinc_payout_token_account =
            PdaHelper::get_round_zinc_payout_token_account_address(round_id, &treasury, &zinc_mint);
        let bonanza_token_account = PdaHelper::get_bonanza_token_account_address();
        CreditRoundRewards {
            signer,
            config: PdaHelper::get_config_address(),
            round: PdaHelper::get_round_address(round_id),
            miner: PdaHelper::get_miner_address(round_id, &player),
            player_profile: PdaHelper::get_player_profile_address(&player),
            treasury,
            zinc_mint,
            round_zinc_payout_token_account,
            bonanza_token_account,
            round_zinc_reward_token_account: PdaHelper::get_round_zinc_reward_token_account_address(
            ),
            token_program: Pubkey::new_from_array(TOKEN_PROGRAM_ID.to_bytes()),
        }
        .instruction()
    }

    /// Builds the SOL claim-round instruction.
    pub fn claim_round_sol_instruction(inputs: ClaimRoundSolInstructionInputs) -> Instruction {
        let ClaimRoundSolInstructionInputs {
            signer,
            player,
            round_id,
        } = inputs;
        ClaimRoundSol {
            signer,
            config: PdaHelper::get_config_address(),
            round: PdaHelper::get_round_address(round_id),
            miner: PdaHelper::get_miner_address(round_id, &player),
            player,
        }
        .instruction()
    }

    /// Builds the aggregate player SOL reward claim instruction.
    pub fn claim_player_sol_rewards_instruction(
        inputs: ClaimPlayerSolRewardsInstructionInputs,
    ) -> Instruction {
        let ClaimPlayerSolRewardsInstructionInputs { signer } = inputs;
        ClaimPlayerSolRewards {
            signer,
            player_profile: PdaHelper::get_player_profile_address(&signer),
        }
        .instruction()
    }

    /// Builds the aggregate player ZINC reward claim instruction.
    pub fn claim_player_zinc_rewards_instruction(
        inputs: ClaimPlayerZincRewardsInstructionInputs,
    ) -> Instruction {
        let ClaimPlayerZincRewardsInstructionInputs { signer, zinc_mint } = inputs;
        let treasury = PdaHelper::get_treasury_address();
        ClaimPlayerZincRewards {
            signer,
            config: PdaHelper::get_config_address(),
            treasury,
            zinc_mint,
            player_profile: PdaHelper::get_player_profile_address(&signer),
            round_zinc_reward_token_account: PdaHelper::get_round_zinc_reward_token_account_address(
            ),
            signer_zinc_token_account: get_player_zinc_token_account(&signer, &zinc_mint),
            associated_token_program: Pubkey::new_from_array(
                ASSOCIATED_TOKEN_PROGRAM_ID.to_bytes(),
            ),
            token_program: Pubkey::new_from_array(TOKEN_PROGRAM_ID.to_bytes()),
            system_program: PdaHelper::get_system_program_address(),
        }
        .instruction()
    }

    /// Backward-compatible alias for the aggregate ZINC reward claim instruction.
    pub fn claim_round_instruction(inputs: ClaimPlayerZincRewardsInstructionInputs) -> Instruction {
        Self::claim_player_zinc_rewards_instruction(inputs)
    }
}
