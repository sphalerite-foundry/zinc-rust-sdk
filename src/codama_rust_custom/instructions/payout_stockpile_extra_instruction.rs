use crate::codama_rust::instructions::{PayoutStockpileExtra, PayoutStockpileExtraInstructionArgs};
use crate::codama_rust_custom::instructions::InstructionsHelper;
use crate::codama_rust_custom::pda::PdaHelper;
use solana_instruction::Instruction;
use solana_pubkey::pubkey;
use solana_pubkey::Pubkey;

const ASSOCIATED_TOKEN_PROGRAM_ID: solana_pubkey::Pubkey =
    pubkey!("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL");
const TOKEN_PROGRAM_ID: solana_pubkey::Pubkey =
    pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");

fn get_classic_ata(owner: &Pubkey, mint: &Pubkey) -> Pubkey {
    Pubkey::find_program_address(
        &[owner.as_ref(), TOKEN_PROGRAM_ID.as_ref(), mint.as_ref()],
        &ASSOCIATED_TOKEN_PROGRAM_ID,
    )
    .0
}

pub struct PayoutStockpileExtraInstructionInputs {
    /// Authorized crank signer.
    pub signer: Pubkey,
    /// Stockpile cycle whose next extra prize is being paid.
    pub stockpile_id: u64,
    /// Resolved stockpile winner that receives the extra prize.
    pub winner: Pubkey,
    /// Classic SPL mint being paid out.
    pub extra_mint: Pubkey,
    /// Zero-based index of the unpaid extra to settle.
    pub extra_index: u16,
}

impl InstructionsHelper {
    pub fn payout_stockpile_extra_instruction(
        inputs: PayoutStockpileExtraInstructionInputs,
    ) -> Instruction {
        let PayoutStockpileExtraInstructionInputs {
            signer,
            stockpile_id,
            winner,
            extra_mint,
            extra_index,
        } = inputs;
        PayoutStockpileExtra {
            signer,
            config: PdaHelper::get_config_address(),
            stockpile: PdaHelper::get_stockpile_address(stockpile_id),
            stockpile_extras: PdaHelper::get_stockpile_extras_address(),
            board: PdaHelper::get_board_address(),
            treasury: PdaHelper::get_treasury_address(),
            extra_mint,
            stockpile_extra_token_account: get_classic_ata(
                &PdaHelper::get_treasury_address(),
                &extra_mint,
            ),
            winner,
            winner_extra_token_account: get_classic_ata(&winner, &extra_mint),
            associated_token_program: Pubkey::new_from_array(
                ASSOCIATED_TOKEN_PROGRAM_ID.to_bytes(),
            ),
            token_program: Pubkey::new_from_array(TOKEN_PROGRAM_ID.to_bytes()),
            system_program: PdaHelper::get_system_program_address(),
        }
        .instruction(PayoutStockpileExtraInstructionArgs { extra_index })
    }
}
