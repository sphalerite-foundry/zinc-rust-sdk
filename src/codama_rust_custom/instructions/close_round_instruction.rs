use crate::codama_rust::instructions::CloseRound;
use crate::codama_rust_custom::instructions::InstructionsHelper;
use crate::codama_rust_custom::pda::PdaHelper;
use solana_instruction::Instruction;
use solana_pubkey::pubkey;
use solana_pubkey::Pubkey;

const TOKEN_PROGRAM_ID: solana_pubkey::Pubkey =
    pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");

pub struct CloseRoundInstructionInputs {
    /// Crank authority that closes the active round.
    pub signer: Pubkey,
    /// Round that should move into settlement.
    pub round_id: u64,
    /// Existing stockpile PDA seed to pass into the instruction.
    pub stockpile_id: u64,
    /// Protocol ZINC mint persisted on the treasury account.
    pub zinc_mint: Pubkey,
    /// Curve-admin token account that receives skimmed ZINC.
    pub curve_admin_token_account: Pubkey,
    /// Bonanza vault that receives rolling Bonanza accrual.
    pub bonanza_token_account: Pubkey,
    /// Stockpile vault that receives stockpile ZINC accrual.
    pub stockpile_token_account: Pubkey,
}

impl InstructionsHelper {
    /// Builds the close-round instruction with the treasury Bonanza mint accounts filled in.
    pub fn close_round_instruction(inputs: CloseRoundInstructionInputs) -> Instruction {
        let CloseRoundInstructionInputs {
            signer,
            round_id,
            stockpile_id,
            zinc_mint,
            curve_admin_token_account,
            bonanza_token_account,
            stockpile_token_account,
        } = inputs;
        let treasury = PdaHelper::get_treasury_address();
        let token_program = Pubkey::new_from_array(TOKEN_PROGRAM_ID.to_bytes());
        let round_zinc_payout_token_account =
            PdaHelper::get_round_zinc_payout_token_account_address(round_id, &treasury, &zinc_mint);
        CloseRound {
            signer,
            config: PdaHelper::get_config_address(),
            board: PdaHelper::get_board_address(),
            round: PdaHelper::get_round_address(round_id),
            treasury,
            zinc_mint,
            curve_admin_token_account,
            bonanza_token_account,
            round_zinc_payout_token_account,
            stockpile_token_account,
            stockpile: PdaHelper::get_stockpile_address(stockpile_id),
            token_program,
            system_program: PdaHelper::get_system_program_address(),
        }
        .instruction()
    }
}
