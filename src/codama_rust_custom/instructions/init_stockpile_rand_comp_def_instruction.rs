use crate::codama_rust::instructions::{
    InitStockpileRandCompDef, InitStockpileRandCompDefInstructionArgs,
};
use crate::codama_rust::types::InitCompDefArgs;
use crate::codama_rust_custom::instructions::InstructionsHelper;
use crate::codama_rust_custom::pda::PdaHelper;
use solana_instruction::Instruction;
use solana_pubkey::Pubkey;

pub struct InitStockpileRandCompDefInstructionInputs {
    pub payer: Pubkey,
    pub address_lookup_table: Pubkey,
    pub source: String,
}

impl InstructionsHelper {
    pub fn init_stockpile_rand_comp_def_instruction(
        inputs: InitStockpileRandCompDefInstructionInputs,
    ) -> Instruction {
        let InitStockpileRandCompDefInstructionInputs {
            payer,
            address_lookup_table,
            source,
        } = inputs;
        InitStockpileRandCompDef {
            payer,
            config: PdaHelper::get_config_address(),
            mxe_account: PdaHelper::get_mxe_account_address(),
            comp_def_account: PdaHelper::get_init_stockpile_rand_comp_def_account_address(),
            address_lookup_table,
            lut_program: PdaHelper::get_lut_program_address(),
            arcium_program: PdaHelper::ARCIUM_PROGRAM_ID,
            system_program: PdaHelper::get_system_program_address(),
        }
        .instruction(InitStockpileRandCompDefInstructionArgs {
            args: InitCompDefArgs { source },
        })
    }
}
