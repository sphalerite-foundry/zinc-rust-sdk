use crate::codama_rust::instructions::{
    Buyback, BuybackInstructionArgs, CreateBuybackPool, CreateBuybackPoolInstructionArgs,
    LockBuybackLiquidity, LockBuybackLiquidityInstructionArgs, RemoveBuybackLiquidity,
    RemoveBuybackLiquidityInstructionArgs, WrapBuybackSol, WrapBuybackSolInstructionArgs,
};
use crate::codama_rust_custom::instructions::InstructionsHelper;
use crate::codama_rust_custom::pda::PdaHelper;
use solana_instruction::{AccountMeta, Instruction};
use solana_pubkey::Pubkey;
use spl_token::instruction as spl_token_instruction;

/// Zinc discriminator for `claim_buyback_pool_fees`.
const CLAIM_BUYBACK_POOL_FEES_DISCRIMINATOR: [u8; 8] = [124, 86, 221, 109, 203, 99, 227, 187];

/// Inputs needed to build the buyback SOL wrapping instruction.
pub struct WrapBuybackSolInstructionInputs {
    /// Crank signer paying any needed ATA rent and submitting the wrap.
    pub signer: Pubkey,
    /// Exact lamports to move from the buyback SOL vault into treasury WSOL.
    pub amount: u64,
}

/// Inputs needed to build the direct Meteora-backed buyback instruction.
pub struct BuybackInstructionInputs {
    /// Crank signer submitting the buyback transaction.
    pub signer: Pubkey,
    /// Protocol ZINC mint persisted on the treasury account.
    pub zinc_mint: Pubkey,
    /// Exact WSOL amount the Meteora swap must spend.
    pub amount_in: u64,
    /// Minimum ZINC amount accepted after the Meteora swap.
    pub min_zinc_out: u64,
    /// Stored Meteora pool authority PDA.
    pub pool_authority: Pubkey,
    /// Stored Meteora pool account.
    pub pool: Pubkey,
    /// Stored Meteora token A vault.
    pub token_a_vault: Pubkey,
    /// Stored Meteora token B vault.
    pub token_b_vault: Pubkey,
    /// Stored Meteora event authority PDA.
    pub event_authority: Pubkey,
}

/// Inputs needed to create and persist the protocol Meteora buyback pool.
pub struct CreateBuybackPoolInstructionInputs {
    /// Admin signer authorized by config.
    pub admin: Pubkey,
    /// Protocol ZINC mint persisted on the treasury account.
    pub zinc_mint: Pubkey,
    /// Meteora static config account used to create the pool.
    pub damm_config: Pubkey,
    /// New Token-2022 mint signer for the initial liquidity position NFT.
    pub position_nft_mint: Pubkey,
    /// Initial ZINC base units minted into the pool.
    pub lp_zinc_amount: u64,
    /// Initial SOL lamports wrapped into WSOL and deposited into the pool.
    pub initial_wsol_lamports: u64,
    /// Initial Meteora liquidity parameter.
    pub liquidity: u128,
    /// Initial sqrt(token_b/token_a) price in Q64.64.
    pub sqrt_price: u128,
    /// Optional Meteora activation point.
    pub activation_point: Option<u64>,
}

/// Inputs needed to build the protocol buyback pool fee-claim instruction.
pub struct ClaimBuybackPoolFeesInstructionInputs {
    /// Configured admin signer submitting the claim transaction.
    pub admin: Pubkey,
    /// Protocol ZINC mint persisted on the treasury account.
    pub zinc_mint: Pubkey,
    /// Stored Meteora pool authority PDA.
    pub pool_authority: Pubkey,
    /// Stored Meteora pool account.
    pub pool: Pubkey,
    /// Stored Meteora initial position account.
    pub position: Pubkey,
    /// Stored Token-2022 account holding the position NFT.
    pub position_nft_account: Pubkey,
    /// Stored Meteora token A vault.
    pub token_a_vault: Pubkey,
    /// Stored Meteora token B vault.
    pub token_b_vault: Pubkey,
    /// Stored Meteora event authority PDA.
    pub event_authority: Pubkey,
}

/// Inputs needed to permanently lock protocol buyback LP liquidity.
pub struct LockBuybackLiquidityInstructionInputs {
    /// Admin signer submitting the LP lock transaction.
    pub admin: Pubkey,
    /// Liquidity amount to permanently lock from the stored Meteora position.
    pub liquidity_delta: u128,
    /// Stored Meteora pool account.
    pub pool: Pubkey,
    /// Stored Meteora initial position account.
    pub position: Pubkey,
    /// Stored Token-2022 account holding the position NFT.
    pub position_nft_account: Pubkey,
    /// Stored Meteora event authority PDA.
    pub event_authority: Pubkey,
}

/// Inputs needed to remove unlocked protocol buyback LP liquidity.
pub struct RemoveBuybackLiquidityInstructionInputs {
    /// Admin signer submitting the LP removal transaction.
    pub admin: Pubkey,
    /// Protocol ZINC mint persisted on the treasury account.
    pub zinc_mint: Pubkey,
    /// Liquidity amount to remove from the stored Meteora position.
    pub liquidity_delta: u128,
    /// Minimum token A amount accepted by the caller.
    pub token_a_amount_threshold: u64,
    /// Minimum token B amount accepted by the caller.
    pub token_b_amount_threshold: u64,
    /// Stored Meteora pool authority PDA.
    pub pool_authority: Pubkey,
    /// Stored Meteora pool account.
    pub pool: Pubkey,
    /// Stored Meteora initial position account.
    pub position: Pubkey,
    /// Stored Token-2022 account holding the position NFT.
    pub position_nft_account: Pubkey,
    /// Stored Meteora token A vault.
    pub token_a_vault: Pubkey,
    /// Stored Meteora token B vault.
    pub token_b_vault: Pubkey,
    /// Stored Meteora event authority PDA.
    pub event_authority: Pubkey,
}

impl InstructionsHelper {
    /// Builds the instruction that wraps buyback SOL into treasury-owned WSOL.
    pub fn wrap_buyback_sol_instruction(inputs: WrapBuybackSolInstructionInputs) -> Instruction {
        let WrapBuybackSolInstructionInputs { signer, amount } = inputs;
        WrapBuybackSol {
            signer,
            config: PdaHelper::get_config_address(),
            treasury: PdaHelper::get_treasury_address(),
            buyback_sol_vault: PdaHelper::get_buyback_sol_vault_address(),
            wsol_mint: PdaHelper::WSOL_MINT_ID,
            treasury_wsol_token_account: PdaHelper::get_treasury_wsol_token_account_address(),
            associated_token_program: PdaHelper::ASSOCIATED_TOKEN_PROGRAM_ID,
            token_program: PdaHelper::TOKEN_PROGRAM_ID,
            system_program: PdaHelper::get_system_program_address(),
        }
        .instruction(WrapBuybackSolInstructionArgs { amount })
    }

    /// Builds the top-level SPL Token instruction that syncs treasury-owned WSOL.
    pub fn sync_treasury_wsol_instruction() -> Instruction {
        Instruction {
            program_id: PdaHelper::TOKEN_PROGRAM_ID,
            accounts: vec![AccountMeta::new(
                PdaHelper::get_treasury_wsol_token_account_address(),
                false,
            )],
            data: spl_token_instruction::TokenInstruction::SyncNative.pack(),
        }
    }

    /// Builds the guarded buyback instruction around the stored Meteora pool.
    pub fn buyback_instruction(inputs: BuybackInstructionInputs) -> Instruction {
        let BuybackInstructionInputs {
            signer,
            zinc_mint,
            amount_in,
            min_zinc_out,
            pool_authority,
            pool,
            token_a_vault,
            token_b_vault,
            event_authority,
        } = inputs;
        Buyback {
            signer,
            config: PdaHelper::get_config_address(),
            treasury: PdaHelper::get_treasury_address(),
            buyback_pool: PdaHelper::get_buyback_pool_address(),
            wsol_mint: PdaHelper::WSOL_MINT_ID,
            zinc_mint,
            treasury_wsol_token_account: PdaHelper::get_treasury_wsol_token_account_address(),
            buyback_zinc_token_account: PdaHelper::get_buyback_zinc_token_account_address(),
            staking_reward_token_account: PdaHelper::get_staking_reward_token_account_address(),
            pool_authority,
            pool,
            token_a_vault,
            token_b_vault,
            event_authority,
            meteora_program: PdaHelper::METEORA_DAMM_V2_PROGRAM_ID,
            instructions_sysvar: PdaHelper::INSTRUCTIONS_SYSVAR_ID,
            token_program: PdaHelper::TOKEN_PROGRAM_ID,
            system_program: PdaHelper::get_system_program_address(),
        }
        .instruction(BuybackInstructionArgs {
            amount_in,
            min_zinc_out,
        })
    }

    /// Builds the instruction that initializes the canonical ZINC/WSOL Meteora pool.
    pub fn create_buyback_pool_instruction(
        inputs: CreateBuybackPoolInstructionInputs,
    ) -> Instruction {
        let CreateBuybackPoolInstructionInputs {
            admin,
            zinc_mint,
            damm_config,
            position_nft_mint,
            lp_zinc_amount,
            initial_wsol_lamports,
            liquidity,
            sqrt_price,
            activation_point,
        } = inputs;
        let meteora_accounts = PdaHelper::get_meteora_buyback_pool_accounts(
            &damm_config,
            &zinc_mint,
            &position_nft_mint,
        );
        CreateBuybackPool {
            admin,
            config: PdaHelper::get_config_address(),
            treasury: PdaHelper::get_treasury_address(),
            buyback_pool: PdaHelper::get_buyback_pool_address(),
            wsol_mint: PdaHelper::WSOL_MINT_ID,
            zinc_mint,
            admin_wsol_token_account: PdaHelper::get_classic_ata(&admin, &PdaHelper::WSOL_MINT_ID),
            admin_zinc_token_account: PdaHelper::get_classic_ata(&admin, &zinc_mint),
            damm_config,
            meteora_program: PdaHelper::METEORA_DAMM_V2_PROGRAM_ID,
            pool_authority: meteora_accounts.pool_authority,
            pool: meteora_accounts.pool,
            position_nft_mint,
            position_nft_account: meteora_accounts.position_nft_account,
            position: meteora_accounts.position,
            token_a_vault: meteora_accounts.token_a_vault,
            token_b_vault: meteora_accounts.token_b_vault,
            event_authority: meteora_accounts.event_authority,
            associated_token_program: PdaHelper::ASSOCIATED_TOKEN_PROGRAM_ID,
            token_program: PdaHelper::TOKEN_PROGRAM_ID,
            token2022_program: PdaHelper::TOKEN_2022_PROGRAM_ID,
            system_program: PdaHelper::get_system_program_address(),
        }
        .instruction(CreateBuybackPoolInstructionArgs {
            lp_zinc_amount,
            initial_wsol_lamports,
            liquidity,
            sqrt_price,
            activation_point,
        })
    }

    /// Builds the instruction that claims fees from the protocol Meteora position.
    pub fn claim_buyback_pool_fees_instruction(
        inputs: ClaimBuybackPoolFeesInstructionInputs,
    ) -> Instruction {
        let ClaimBuybackPoolFeesInstructionInputs {
            admin,
            zinc_mint,
            pool_authority,
            pool,
            position,
            position_nft_account,
            token_a_vault,
            token_b_vault,
            event_authority,
        } = inputs;
        Instruction {
            program_id: crate::codama_rust::ZINC_ID,
            accounts: vec![
                AccountMeta::new(admin, true),
                AccountMeta::new_readonly(PdaHelper::get_config_address(), false),
                AccountMeta::new_readonly(PdaHelper::get_treasury_address(), false),
                AccountMeta::new_readonly(PdaHelper::get_buyback_pool_address(), false),
                AccountMeta::new_readonly(PdaHelper::WSOL_MINT_ID, false),
                AccountMeta::new_readonly(zinc_mint, false),
                AccountMeta::new(
                    PdaHelper::get_buyback_fee_zinc_token_account_address(),
                    false,
                ),
                AccountMeta::new(
                    PdaHelper::get_buyback_fee_wsol_token_account_address(),
                    false,
                ),
                AccountMeta::new(PdaHelper::get_classic_ata(&admin, &zinc_mint), false),
                AccountMeta::new(
                    PdaHelper::get_classic_ata(&admin, &PdaHelper::WSOL_MINT_ID),
                    false,
                ),
                AccountMeta::new_readonly(pool_authority, false),
                AccountMeta::new_readonly(pool, false),
                AccountMeta::new(position, false),
                AccountMeta::new_readonly(position_nft_account, false),
                AccountMeta::new(token_a_vault, false),
                AccountMeta::new(token_b_vault, false),
                AccountMeta::new_readonly(event_authority, false),
                AccountMeta::new_readonly(PdaHelper::METEORA_DAMM_V2_PROGRAM_ID, false),
                AccountMeta::new_readonly(PdaHelper::ASSOCIATED_TOKEN_PROGRAM_ID, false),
                AccountMeta::new_readonly(PdaHelper::TOKEN_PROGRAM_ID, false),
                AccountMeta::new_readonly(PdaHelper::get_system_program_address(), false),
            ],
            data: CLAIM_BUYBACK_POOL_FEES_DISCRIMINATOR.to_vec(),
        }
    }

    /// Builds the instruction that permanently locks protocol buyback LP principal.
    pub fn lock_buyback_liquidity_instruction(
        inputs: LockBuybackLiquidityInstructionInputs,
    ) -> Instruction {
        let LockBuybackLiquidityInstructionInputs {
            admin,
            liquidity_delta,
            pool,
            position,
            position_nft_account,
            event_authority,
        } = inputs;
        LockBuybackLiquidity {
            admin,
            config: PdaHelper::get_config_address(),
            treasury: PdaHelper::get_treasury_address(),
            buyback_pool: PdaHelper::get_buyback_pool_address(),
            pool,
            position,
            position_nft_account,
            event_authority,
            meteora_program: PdaHelper::METEORA_DAMM_V2_PROGRAM_ID,
        }
        .instruction(LockBuybackLiquidityInstructionArgs { liquidity_delta })
    }

    /// Builds the instruction that removes unlocked protocol buyback LP principal.
    pub fn remove_buyback_liquidity_instruction(
        inputs: RemoveBuybackLiquidityInstructionInputs,
    ) -> Instruction {
        let RemoveBuybackLiquidityInstructionInputs {
            admin,
            zinc_mint,
            liquidity_delta,
            token_a_amount_threshold,
            token_b_amount_threshold,
            pool_authority,
            pool,
            position,
            position_nft_account,
            token_a_vault,
            token_b_vault,
            event_authority,
        } = inputs;
        RemoveBuybackLiquidity {
            admin,
            config: PdaHelper::get_config_address(),
            treasury: PdaHelper::get_treasury_address(),
            buyback_pool: PdaHelper::get_buyback_pool_address(),
            wsol_mint: PdaHelper::WSOL_MINT_ID,
            zinc_mint,
            buyback_lp_zinc_token_account: PdaHelper::get_buyback_lp_zinc_token_account_address(),
            buyback_lp_wsol_token_account: PdaHelper::get_buyback_lp_wsol_token_account_address(),
            pool_authority,
            pool,
            position,
            position_nft_account,
            token_a_vault,
            token_b_vault,
            event_authority,
            meteora_program: PdaHelper::METEORA_DAMM_V2_PROGRAM_ID,
            token_program: PdaHelper::TOKEN_PROGRAM_ID,
            system_program: PdaHelper::get_system_program_address(),
        }
        .instruction(RemoveBuybackLiquidityInstructionArgs {
            liquidity_delta,
            token_a_amount_threshold,
            token_b_amount_threshold,
        })
    }
}

#[cfg(test)]
#[path = "buyback_instruction_tests.rs"]
mod tests;
