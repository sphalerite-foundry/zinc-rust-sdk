use crate::codama_rust::instructions::{DeployRound, DeployRoundInstructionArgs};
use crate::codama_rust_custom::instructions::InstructionsHelper;
use crate::codama_rust_custom::pda::PdaHelper;
use solana_instruction::Instruction;
use solana_pubkey::Pubkey;

pub struct DeployRoundInstructionInputs {
    pub signer: Pubkey,
    pub round_id: u64,
    /// Active stockpile id to pass into the optional deploy account set.
    pub stockpile_id: Option<u64>,
    pub total_amount: u64,
    /// Optional affiliate account to bind on the player's immutable first deploy.
    pub affiliate: Option<Pubkey>,
    pub mask_encryption_key: [u8; 32],
    pub mask_nonce: u128,
    pub mask_ciphertext: [u8; 64],
}

#[cfg(test)]
#[path = "deploy_round_instruction_tests.rs"]
mod tests;

impl InstructionsHelper {
    pub fn deploy_round_instruction(inputs: DeployRoundInstructionInputs) -> Instruction {
        let DeployRoundInstructionInputs {
            signer,
            round_id,
            stockpile_id,
            total_amount,
            affiliate,
            mask_encryption_key,
            mask_nonce,
            mask_ciphertext,
        } = inputs;
        let effective_affiliate = affiliate.filter(|candidate| *candidate != signer);
        let affiliate_profile =
            effective_affiliate.map(|affiliate| PdaHelper::get_player_profile_address(&affiliate));
        DeployRound {
            signer,
            round: PdaHelper::get_round_address(round_id),
            config: PdaHelper::get_config_address(),
            miner: PdaHelper::get_miner_address(round_id, &signer),
            player_profile: PdaHelper::get_player_profile_address(&signer),
            board: PdaHelper::get_board_address(),
            treasury: PdaHelper::get_treasury_address(),
            stockpile_sol_vault: PdaHelper::get_stockpile_sol_vault_address(),
            buyback_sol_vault: PdaHelper::get_buyback_sol_vault_address(),
            stockpile: stockpile_id
                .map(|stockpile_id| PdaHelper::get_stockpile_address(stockpile_id)),
            affiliate: effective_affiliate,
            affiliate_profile,
            system_program: PdaHelper::get_system_program_address(),
        }
        .instruction(DeployRoundInstructionArgs {
            total_amount,
            mask_encryption_key,
            mask_nonce,
            mask_ciphertext,
        })
    }
}
