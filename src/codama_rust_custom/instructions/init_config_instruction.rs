use crate::codama_rust::instructions::{InitConfig, InitConfigInstructionArgs};
use crate::codama_rust::types::ZincMintMetadataArgs;
use crate::codama_rust_custom::instructions::InstructionsHelper;
use crate::codama_rust_custom::pda::PdaHelper;
use solana_instruction::Instruction;
use solana_pubkey::pubkey;
use solana_pubkey::Pubkey;

const ASSOCIATED_TOKEN_PROGRAM_ID: solana_pubkey::Pubkey =
    pubkey!("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL");
const TOKEN_PROGRAM_ID: solana_pubkey::Pubkey =
    pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");

pub struct InitConfigInstructionInputs {
    /// Admin that pays for config bootstrap.
    pub admin: Pubkey,
    /// Crank authority persisted on config and board state.
    pub crank: Pubkey,
    /// Fresh ZINC mint account created during config bootstrap.
    pub zinc_mint: Pubkey,
    /// Required Metaplex metadata fields for the fresh ZINC mint.
    pub metadata: ZincMintMetadataArgs,
}

impl InstructionsHelper {
    /// Builds the instruction that initializes config, treasury, mint, vaults, and metadata.
    pub fn init_config_instruction(inputs: InitConfigInstructionInputs) -> Instruction {
        let InitConfigInstructionInputs {
            admin,
            crank,
            zinc_mint,
            metadata,
        } = inputs;
        let treasury = PdaHelper::get_treasury_address();
        let zinc_metadata = PdaHelper::get_mint_metadata_address(&zinc_mint);
        let token_program = Pubkey::new_from_array(TOKEN_PROGRAM_ID.to_bytes());
        let associated_token_program =
            Pubkey::new_from_array(ASSOCIATED_TOKEN_PROGRAM_ID.to_bytes());
        let curve_admin_token_account = Pubkey::find_program_address(
            &[
                treasury.as_ref(),
                token_program.as_ref(),
                zinc_mint.as_ref(),
            ],
            &associated_token_program,
        )
        .0;
        let bonanza_token_account = PdaHelper::get_bonanza_token_account_address();
        let stockpile_token_account = PdaHelper::get_stockpile_token_account_address();
        let round_zinc_reward_token_account =
            PdaHelper::get_round_zinc_reward_token_account_address();
        let staking_token_account = PdaHelper::get_staking_token_account_address();
        let staking_reward_token_account = PdaHelper::get_staking_reward_token_account_address();
        let stockpile_sol_vault = PdaHelper::get_stockpile_sol_vault_address();
        let buyback_sol_vault = PdaHelper::get_buyback_sol_vault_address();
        let stockpile_extras = PdaHelper::get_stockpile_extras_address();
        InitConfig {
            admin,
            crank,
            config: PdaHelper::get_config_address(),
            treasury,
            zinc_mint,
            zinc_metadata,
            curve_admin_token_account,
            bonanza_token_account,
            stockpile_token_account,
            round_zinc_reward_token_account,
            staking_token_account,
            staking_reward_token_account,
            stockpile_sol_vault,
            buyback_sol_vault,
            stockpile_extras,
            associated_token_program,
            token_program,
            metadata_program: PdaHelper::METADATA_PROGRAM_ID,
            rent: PdaHelper::RENT_SYSVAR_ID,
            system_program: PdaHelper::get_system_program_address(),
        }
        .instruction(InitConfigInstructionArgs { args: metadata })
    }
}
