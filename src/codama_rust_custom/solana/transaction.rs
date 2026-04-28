use crate::codama_rust_custom::solana::{to_sdk_instruction, SolanaHelper};
use anyhow::anyhow;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_client::rpc_config::RpcSendTransactionConfig;
use solana_commitment_config::CommitmentConfig;
use solana_compute_budget_interface::ComputeBudgetInstruction;
use solana_message::{v0, AddressLookupTableAccount, VersionedMessage};
use solana_sdk::signature::{Keypair, Signature, Signer};
use solana_sdk::transaction::VersionedTransaction;
use std::str::FromStr;
use tokio::time::{sleep, Duration};

impl SolanaHelper {
    const CONFIRMATION_POLL_INTERVAL_MS: u64 = 500;
    const CONFIRMATION_RETRIES: u8 = 10;

    /// Sends a v0 transaction signed only by the payer keypair.
    pub async fn send_transaction(
        rpc: &RpcClient,
        signer: &Keypair,
        instructions: Vec<solana_instruction::Instruction>,
        lookup_tables: &[AddressLookupTableAccount],
    ) -> anyhow::Result<String> {
        Self::send_transaction_with_signers(rpc, signer, &[], instructions, lookup_tables).await
    }

    /// Sends a v0 transaction signed by the payer plus any additional signer keypairs.
    pub async fn send_transaction_with_signers(
        rpc: &RpcClient,
        signer: &Keypair,
        additional_signers: &[&Keypair],
        instructions: Vec<solana_instruction::Instruction>,
        lookup_tables: &[AddressLookupTableAccount],
    ) -> anyhow::Result<String> {
        let confirmed_commitment = CommitmentConfig::confirmed();
        let compute_budget = ComputeBudgetInstruction::set_compute_unit_limit(3_000_000);
        let recent_blockhash = rpc.get_latest_blockhash().await?;
        let mut tx_instructions = vec![compute_budget];
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
                .get_signature_statuses(&[sig])
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
                    "signature {signature} did not reach confirmed commitment after {} attempts",
                    Self::CONFIRMATION_RETRIES
                ));
            }
            sleep(Duration::from_millis(Self::CONFIRMATION_POLL_INTERVAL_MS)).await;
        }
    }
}
