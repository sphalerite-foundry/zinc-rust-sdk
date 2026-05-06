use crate::codama_rust_custom::solana::{to_sdk_instruction, SolanaHelper};
use anyhow::anyhow;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_client::rpc_config::RpcSendTransactionConfig;
use solana_commitment_config::CommitmentConfig;
use solana_compute_budget_interface::ComputeBudgetInstruction;
use solana_message::{v0, AddressLookupTableAccount, VersionedMessage};
use solana_sdk::signature::{Keypair, Signature, Signer};
use solana_sdk::transaction::VersionedTransaction;
use std::{env, str::FromStr};
use tokio::time::{sleep, Duration};

/// Priority-fee tier used when sending a Zinc transaction.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PriorityFeeTier {
    /// Latency-sensitive protocol work that should land quickly for user-facing flows.
    Urgent,
    /// Best-effort maintenance work that can tolerate lower priority fees.
    Background,
}

impl SolanaHelper {
    const CONFIRMATION_POLL_INTERVAL_MS: u64 = 1_000;
    const CONFIRMATION_RETRIES: u8 = 10;
    const LEGACY_COMPUTE_UNIT_PRICE_ENV: &str = "ZINC_COMPUTE_UNIT_PRICE_MICRO_LAMPORTS";
    const URGENT_COMPUTE_UNIT_PRICE_ENV: &str = "ZINC_URGENT_COMPUTE_UNIT_PRICE_MICRO_LAMPORTS";
    const BACKGROUND_COMPUTE_UNIT_PRICE_ENV: &str =
        "ZINC_BACKGROUND_COMPUTE_UNIT_PRICE_MICRO_LAMPORTS";
    const DEFAULT_URGENT_COMPUTE_UNIT_PRICE_MICRO_LAMPORTS: u64 = 10_000;
    const DEFAULT_BACKGROUND_COMPUTE_UNIT_PRICE_MICRO_LAMPORTS: u64 = 2_500;

    /// Sends a v0 transaction signed only by the payer keypair.
    pub async fn send_transaction(
        rpc: &RpcClient,
        signer: &Keypair,
        instructions: Vec<solana_instruction::Instruction>,
        lookup_tables: &[AddressLookupTableAccount],
    ) -> anyhow::Result<String> {
        Self::send_transaction_with_signers(rpc, signer, &[], instructions, lookup_tables).await
    }

    /// Sends a v0 transaction signed only by the payer keypair with the selected fee tier.
    pub async fn send_transaction_with_fee_tier(
        rpc: &RpcClient,
        signer: &Keypair,
        instructions: Vec<solana_instruction::Instruction>,
        lookup_tables: &[AddressLookupTableAccount],
        fee_tier: PriorityFeeTier,
    ) -> anyhow::Result<String> {
        Self::send_transaction_with_signers_and_fee_tier(
            rpc,
            signer,
            &[],
            instructions,
            lookup_tables,
            fee_tier,
        )
        .await
    }

    /// Sends a v0 transaction signed by the payer plus any additional signer keypairs.
    pub async fn send_transaction_with_signers(
        rpc: &RpcClient,
        signer: &Keypair,
        additional_signers: &[&Keypair],
        instructions: Vec<solana_instruction::Instruction>,
        lookup_tables: &[AddressLookupTableAccount],
    ) -> anyhow::Result<String> {
        Self::send_transaction_with_signers_and_fee_tier(
            rpc,
            signer,
            additional_signers,
            instructions,
            lookup_tables,
            PriorityFeeTier::Urgent,
        )
        .await
    }

    /// Sends a v0 transaction signed by the payer plus additional signer keypairs with the selected fee tier.
    pub async fn send_transaction_with_signers_and_fee_tier(
        rpc: &RpcClient,
        signer: &Keypair,
        additional_signers: &[&Keypair],
        instructions: Vec<solana_instruction::Instruction>,
        lookup_tables: &[AddressLookupTableAccount],
        fee_tier: PriorityFeeTier,
    ) -> anyhow::Result<String> {
        let confirmed_commitment = CommitmentConfig::confirmed();
        let compute_budget = ComputeBudgetInstruction::set_compute_unit_limit(3_000_000);
        let compute_unit_price = Self::compute_unit_price_micro_lamports(fee_tier)?;
        let recent_blockhash = rpc.get_latest_blockhash().await?;
        let mut tx_instructions = vec![compute_budget];
        if let Some(compute_unit_price) = compute_unit_price {
            tx_instructions.push(ComputeBudgetInstruction::set_compute_unit_price(
                compute_unit_price,
            ));
        }
        tx_instructions.extend(instructions.into_iter().map(to_sdk_instruction));
        let message = v0::Message::try_compile(
            &signer.pubkey(),
            &tx_instructions,
            lookup_tables,
            recent_blockhash,
        )
        .map_err(|error| anyhow!("failed to compile v0 message: {error}"))?;
        let versioned_message = VersionedMessage::V0(message);
        let transaction = {
            let mut signers: Vec<&dyn Signer> = vec![signer];
            signers.extend(
                additional_signers
                    .iter()
                    .copied()
                    .map(|extra_signer| extra_signer as &dyn Signer),
            );
            VersionedTransaction::try_new(versioned_message, &signers)
                .map_err(|error| anyhow!("failed to create versioned transaction: {error}"))?
        };
        let signature = transaction.signatures[0].to_string();
        rpc.send_transaction_with_config(
            &transaction,
            RpcSendTransactionConfig {
                preflight_commitment: Some(confirmed_commitment.commitment),
                ..RpcSendTransactionConfig::default()
            },
        )
        .await
        .map_err(|error| anyhow!("signature {signature} failed with error: {error}"))?;
        Self::confirm_transaction(rpc, &signature).await
    }

    pub async fn confirm_transaction(rpc: &RpcClient, signature: &str) -> anyhow::Result<String> {
        let confirmed_commitment = CommitmentConfig::confirmed();
        let mut count = 0;
        let sig = Signature::from_str(signature)?;
        loop {
            let maybe_status = rpc
                .get_signature_statuses_with_history(&[sig])
                .await?
                .value
                .get(0)
                .ok_or(anyhow!("No signature returned"))
                .cloned()?;
            if let Some(status) = maybe_status {
                if let Some(error) = status.err {
                    return Err(anyhow!("signature {signature} failed with error: {error}"));
                }
                if status.satisfies_commitment(confirmed_commitment) {
                    return Ok(signature.to_string());
                }
            }
            count += 1;
            if count >= Self::CONFIRMATION_RETRIES {
                return Err(anyhow!(
                    "signature {signature} did not reach confirmed commitment after {} attempts over roughly {} seconds",
                    Self::CONFIRMATION_RETRIES,
                    u64::from(Self::CONFIRMATION_RETRIES)
                        * Self::CONFIRMATION_POLL_INTERVAL_MS
                        / 1_000
                ));
            }
            sleep(Duration::from_millis(Self::CONFIRMATION_POLL_INTERVAL_MS)).await;
        }
    }

    /// Loads the configured compute-unit price for a priority-fee tier.
    fn compute_unit_price_micro_lamports(fee_tier: PriorityFeeTier) -> anyhow::Result<Option<u64>> {
        let (env_name, raw_value) = Self::compute_unit_price_raw_value(fee_tier);
        let trimmed_value = raw_value.trim();
        let price = trimmed_value
            .parse::<u64>()
            .map_err(|error| anyhow!("invalid {env_name} value {trimmed_value:?}: {error}"))?;

        Ok((price > 0).then_some(price))
    }

    /// Resolves the raw configured compute-unit price for a priority-fee tier.
    fn compute_unit_price_raw_value(fee_tier: PriorityFeeTier) -> (&'static str, String) {
        match fee_tier {
            PriorityFeeTier::Urgent => {
                if let Ok(raw_value) = env::var(Self::URGENT_COMPUTE_UNIT_PRICE_ENV) {
                    return (Self::URGENT_COMPUTE_UNIT_PRICE_ENV, raw_value);
                }
                if let Ok(raw_value) = env::var(Self::LEGACY_COMPUTE_UNIT_PRICE_ENV) {
                    return (Self::LEGACY_COMPUTE_UNIT_PRICE_ENV, raw_value);
                }
                (
                    Self::URGENT_COMPUTE_UNIT_PRICE_ENV,
                    Self::DEFAULT_URGENT_COMPUTE_UNIT_PRICE_MICRO_LAMPORTS.to_string(),
                )
            }
            PriorityFeeTier::Background => {
                let raw_value =
                    env::var(Self::BACKGROUND_COMPUTE_UNIT_PRICE_ENV).unwrap_or_else(|_| {
                        Self::DEFAULT_BACKGROUND_COMPUTE_UNIT_PRICE_MICRO_LAMPORTS.to_string()
                    });
                (Self::BACKGROUND_COMPUTE_UNIT_PRICE_ENV, raw_value)
            }
        }
    }
}
