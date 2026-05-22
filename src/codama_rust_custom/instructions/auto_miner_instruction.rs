use crate::codama_rust::instructions::{
    DeployRoundFromAutoSession, DeployRoundFromAutoSessionInstructionArgs,
    ReloadAutoMinerSessionSol,
};
use crate::codama_rust_custom::instructions::InstructionsHelper;
use crate::codama_rust_custom::pda::PdaHelper;
use solana_instruction::Instruction;
use solana_pubkey::Pubkey;

pub struct DeployRoundFromAutoSessionInstructionInputs {
    /// Configured crank signer executing the auto deploy.
    pub executor: Pubkey,
    /// Wallet that owns the session and receives the miner position.
    pub authority: Pubkey,
    /// Active stockpile id to pass into the optional deploy account set.
    pub stockpile_id: Option<u64>,
    /// Round id receiving the auto deploy.
    pub round_id: u64,
    /// Optional affiliate wallet forwarded only for already-bound affiliate accrual.
    pub affiliate: Option<Pubkey>,
    /// X25519 public key used to encrypt the hidden tile mask for Arcium.
    pub mask_encryption_key: [u8; 32],
    /// Rescue CTR nonce used for the encrypted tile mask payload.
    pub mask_nonce: u128,
    /// Ciphertext limbs for the encrypted hidden tile mask payload.
    pub mask_ciphertext: [u8; 64],
}

pub struct ReloadAutoMinerSessionSolInstructionInputs {
    /// Configured crank signer executing the reward reload.
    pub executor: Pubkey,
    /// Wallet that owns the session being reloaded.
    pub authority: Pubkey,
}

impl InstructionsHelper {
    /// Builds one crank-executed auto-miner deploy instruction.
    pub fn deploy_round_from_auto_session_instruction(
        inputs: DeployRoundFromAutoSessionInstructionInputs,
    ) -> Instruction {
        let DeployRoundFromAutoSessionInstructionInputs {
            executor,
            authority,
            stockpile_id,
            round_id,
            affiliate,
            mask_encryption_key,
            mask_nonce,
            mask_ciphertext,
        } = inputs;
        let affiliate_profile =
            affiliate.map(|affiliate| PdaHelper::get_player_profile_address(&affiliate));
        DeployRoundFromAutoSession {
            executor,
            authority,
            auto_miner_session: PdaHelper::get_auto_miner_session_address(&authority),
            round: PdaHelper::get_round_address(round_id),
            config: PdaHelper::get_config_address(),
            miner: PdaHelper::get_miner_address(round_id, &authority),
            player_profile: PdaHelper::get_player_profile_address(&authority),
            board: PdaHelper::get_board_address(),
            treasury: PdaHelper::get_treasury_address(),
            stockpile_sol_vault: PdaHelper::get_stockpile_sol_vault_address(),
            buyback_sol_vault: PdaHelper::get_buyback_sol_vault_address(),
            stockpile: stockpile_id
                .map(|stockpile_id| PdaHelper::get_stockpile_address(stockpile_id)),
            affiliate,
            affiliate_profile,
            system_program: solana_system_interface::program::ID,
        }
        .instruction(DeployRoundFromAutoSessionInstructionArgs {
            mask_encryption_key,
            mask_nonce,
            mask_ciphertext,
        })
    }

    /// Builds one crank-executed auto-miner reward reload instruction.
    pub fn reload_auto_miner_session_sol_instruction(
        inputs: ReloadAutoMinerSessionSolInstructionInputs,
    ) -> Instruction {
        let ReloadAutoMinerSessionSolInstructionInputs {
            executor,
            authority,
        } = inputs;
        ReloadAutoMinerSessionSol {
            executor,
            config: PdaHelper::get_config_address(),
            authority,
            auto_miner_session: PdaHelper::get_auto_miner_session_address(&authority),
            player_profile: PdaHelper::get_player_profile_address(&authority),
        }
        .instruction()
    }
}
