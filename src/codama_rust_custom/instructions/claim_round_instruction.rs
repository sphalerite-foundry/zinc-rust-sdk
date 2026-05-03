use crate::codama_rust::instructions::{ClaimRoundSol, ClaimRoundZinc};
use crate::codama_rust_custom::instructions::InstructionsHelper;
use crate::codama_rust_custom::pda::PdaHelper;
use solana_instruction::Instruction;
use solana_pubkey::pubkey;
use solana_pubkey::Pubkey;

const ASSOCIATED_TOKEN_PROGRAM_ID: solana_pubkey::Pubkey =
    pubkey!("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL");
const TOKEN_PROGRAM_ID: solana_pubkey::Pubkey =
    pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");

pub struct ClaimRoundInstructionInputs {
    /// Crank or player signer submitting the claim transaction.
    pub signer: Pubkey,
    /// Claim recipient whose miner PDA and ATA are derived.
    pub player: Pubkey,
    /// Settled round being claimed.
    pub round_id: u64,
    /// Protocol ZINC mint persisted on the treasury account.
    pub zinc_mint: Pubkey,
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
    /// Builds the ZINC claim-round instruction with treasury and player ATA accounts resolved.
    pub fn claim_round_zinc_instruction(inputs: ClaimRoundInstructionInputs) -> Instruction {
        let ClaimRoundInstructionInputs {
            signer,
            player,
            round_id,
            zinc_mint,
        } = inputs;
        let treasury = PdaHelper::get_treasury_address();
        let round_zinc_payout_token_account =
            PdaHelper::get_round_zinc_payout_token_account_address(round_id, &treasury, &zinc_mint);
        let bonanza_token_account = PdaHelper::get_bonanza_token_account_address();
        ClaimRoundZinc {
            signer,
            round: PdaHelper::get_round_address(round_id),
            config: PdaHelper::get_config_address(),
            miner: PdaHelper::get_miner_address(round_id, &player),
            player,
            treasury,
            zinc_mint,
            round_zinc_payout_token_account,
            bonanza_token_account,
            player_zinc_token_account: get_player_zinc_token_account(&player, &zinc_mint),
            associated_token_program: Pubkey::new_from_array(
                ASSOCIATED_TOKEN_PROGRAM_ID.to_bytes(),
            ),
            token_program: Pubkey::new_from_array(TOKEN_PROGRAM_ID.to_bytes()),
            system_program: PdaHelper::get_system_program_address(),
        }
        .instruction()
    }

    /// Builds the SOL claim-round instruction.
    pub fn claim_round_sol_instruction(inputs: ClaimRoundInstructionInputs) -> Instruction {
        let ClaimRoundInstructionInputs {
            signer,
            player,
            round_id,
            zinc_mint: _,
        } = inputs;
        ClaimRoundSol {
            signer,
            round: PdaHelper::get_round_address(round_id),
            miner: PdaHelper::get_miner_address(round_id, &player),
            player,
        }
        .instruction()
    }

    /// Backward-compatible alias for the ZINC-only split claim instruction.
    pub fn claim_round_instruction(inputs: ClaimRoundInstructionInputs) -> Instruction {
        Self::claim_round_zinc_instruction(inputs)
    }
}
