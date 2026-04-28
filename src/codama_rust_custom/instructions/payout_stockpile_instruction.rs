use crate::codama_rust::instructions::PayoutStockpile;
use crate::codama_rust_custom::instructions::InstructionsHelper;
use crate::codama_rust_custom::pda::PdaHelper;
use solana_instruction::Instruction;
use solana_pubkey::pubkey;
use solana_pubkey::Pubkey;

const ASSOCIATED_TOKEN_PROGRAM_ID: solana_pubkey::Pubkey =
    pubkey!("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL");
const TOKEN_PROGRAM_ID: solana_pubkey::Pubkey =
    pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");

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

pub struct PayoutStockpileInstructionInputs {
    /// Authorized crank signer.
    pub signer: Pubkey,
    /// Stockpile cycle being paid out.
    pub stockpile_id: u64,
    /// Resolved stockpile winner that receives SOL and ZINC.
    pub winner: Pubkey,
    /// Protocol ZINC mint used to derive the winner ATA.
    pub zinc_mint: Pubkey,
}

impl InstructionsHelper {
    pub fn payout_stockpile_instruction(inputs: PayoutStockpileInstructionInputs) -> Instruction {
        let PayoutStockpileInstructionInputs {
            signer,
            stockpile_id,
            winner,
            zinc_mint,
        } = inputs;
        PayoutStockpile {
            signer,
            config: PdaHelper::get_config_address(),
            stockpile: PdaHelper::get_stockpile_address(stockpile_id),
            stockpile_extras: PdaHelper::get_stockpile_extras_address(),
            board: PdaHelper::get_board_address(),
            treasury: PdaHelper::get_treasury_address(),
            stockpile_sol_vault: PdaHelper::get_stockpile_sol_vault_address(),
            zinc_mint,
            stockpile_token_account: PdaHelper::get_stockpile_token_account_address(),
            winner,
            winner_zinc_token_account: get_player_zinc_token_account(&winner, &zinc_mint),
            associated_token_program: Pubkey::new_from_array(
                ASSOCIATED_TOKEN_PROGRAM_ID.to_bytes(),
            ),
            token_program: Pubkey::new_from_array(TOKEN_PROGRAM_ID.to_bytes()),
            system_program: PdaHelper::get_system_program_address(),
        }
        .instruction()
    }
}
