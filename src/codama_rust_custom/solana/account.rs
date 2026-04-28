use crate::codama_rust::accounts::{
    Board, Miner, Round, Stockpile, MINER_DISCRIMINATOR, ROUND_DISCRIMINATOR,
    STOCKPILE_DISCRIMINATOR,
};
use crate::codama_rust::types::{RoundStatus, StockpileStatus};
use crate::codama_rust_custom::pda::PdaHelper;
use crate::codama_rust_custom::solana::{to_sdk_pubkey, SolanaHelper};
use anyhow::{anyhow, Context, Result};
use borsh::{to_vec, BorshDeserialize};
use solana_account_decoder_client_types::UiAccountEncoding;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_client::rpc_config::{RpcAccountInfoConfig, RpcProgramAccountsConfig};
use solana_client::rpc_filter::{Memcmp, RpcFilterType};
use solana_commitment_config::CommitmentConfig;
use solana_sdk::account::Account;
use solana_sdk::pubkey::Pubkey;

#[derive(Debug, Clone)]
pub struct DecodedAccount<T> {
    /// Address used to fetch the decoded account.
    pub address: Pubkey,
    /// Raw Solana account data returned by RPC.
    pub account: Account,
    /// Borsh-decoded account payload.
    pub data: T,
}

impl SolanaHelper {
    /// Decodes one fetched account into the requested Borsh type.
    fn decode_account<T>(address: &Pubkey, account: Account) -> Result<DecodedAccount<T>>
    where
        T: BorshDeserialize,
    {
        let mut data = account.data.as_slice();
        let decoded = T::deserialize(&mut data).with_context(|| {
            format!(
                "failed to deserialize account {address} into {}",
                std::any::type_name::<T>()
            )
        })?;
        Ok(DecodedAccount {
            address: *address,
            account,
            data: decoded,
        })
    }

    /// Fetches one account and errors when the account does not exist.
    pub async fn fetch_account<T>(rpc: &RpcClient, address: &Pubkey) -> Result<DecodedAccount<T>>
    where
        T: BorshDeserialize,
    {
        let account = rpc
            .get_account_with_commitment(address, CommitmentConfig::confirmed())
            .await
            .map_err(|err| {
                anyhow!(
                    "failed to fetch account {address} from RPC at confirmed commitment: {err:#}"
                )
            })?
            .value
            .ok_or_else(|| anyhow!("account {address} not found at confirmed commitment"))?;
        Self::decode_account(address, account)
    }

    /// Fetches one account and returns `None` when the account does not exist.
    pub async fn fetch_optional_account<T>(
        rpc: &RpcClient,
        address: &Pubkey,
    ) -> Result<Option<DecodedAccount<T>>>
    where
        T: BorshDeserialize,
    {
        let maybe_account = rpc
            .get_account_with_commitment(address, CommitmentConfig::confirmed())
            .await
            .map_err(|err| {
                anyhow!(
                    "failed to fetch account {address} from RPC at confirmed commitment: {err:#}"
                )
            })?
            .value;
        maybe_account
            .map(|account| Self::decode_account(address, account))
            .transpose()
    }

    /// Fetches multiple accounts and errors when any requested account is missing.
    pub async fn fetch_all_accounts<T>(
        rpc: &RpcClient,
        addresses: &[Pubkey],
    ) -> Result<Vec<DecodedAccount<T>>>
    where
        T: BorshDeserialize,
    {
        let accounts = rpc
            .get_multiple_accounts_with_commitment(addresses, CommitmentConfig::confirmed())
            .await
            .map_err(|err| {
                anyhow!(
                    "failed to fetch {} accounts from RPC at confirmed commitment: {err:#}",
                    addresses.len(),
                )
            })?
            .value;
        let mut decoded_accounts = Vec::with_capacity(addresses.len());
        for (address, maybe_account) in addresses.iter().zip(accounts.into_iter()) {
            let account = maybe_account
                .ok_or_else(|| anyhow!("account {address} not found at confirmed commitment"))?;
            decoded_accounts.push(Self::decode_account(address, account)?);
        }
        Ok(decoded_accounts)
    }

    /// Fetches multiple accounts and preserves missing entries as `None`.
    pub async fn fetch_optional_accounts<T>(
        rpc: &RpcClient,
        addresses: &[Pubkey],
    ) -> Result<Vec<Option<DecodedAccount<T>>>>
    where
        T: BorshDeserialize,
    {
        let accounts = rpc
            .get_multiple_accounts_with_commitment(addresses, CommitmentConfig::confirmed())
            .await
            .map_err(|err| {
                anyhow!(
                    "failed to fetch {} accounts from RPC at confirmed commitment: {err:#}",
                    addresses.len(),
                )
            })?
            .value;
        let mut decoded_accounts = Vec::with_capacity(addresses.len());
        for (address, maybe_account) in addresses.iter().zip(accounts.into_iter()) {
            decoded_accounts.push(
                maybe_account
                    .map(|account| Self::decode_account(address, account))
                    .transpose()?,
            );
        }
        Ok(decoded_accounts)
    }

    pub async fn fetch_board(rpc: &RpcClient) -> Result<Board> {
        let board_address = to_sdk_pubkey(PdaHelper::get_board_address());
        Ok(Self::fetch_account::<Board>(rpc, &board_address)
            .await?
            .data)
    }

    pub async fn fetch_rounds_by_status(
        rpc: &RpcClient,
        program_id: &Pubkey,
        status: RoundStatus,
    ) -> Result<Vec<DecodedAccount<Round>>> {
        let status_bytes = to_vec(&status)?;
        let accounts = rpc
            .get_program_accounts_with_config(
                program_id,
                RpcProgramAccountsConfig {
                    filters: Some(vec![
                        RpcFilterType::Memcmp(Memcmp::new_raw_bytes(
                            0,
                            ROUND_DISCRIMINATOR.to_vec(),
                        )),
                        RpcFilterType::Memcmp(Memcmp::new_raw_bytes(8, status_bytes)),
                    ]),
                    account_config: RpcAccountInfoConfig {
                        encoding: Some(UiAccountEncoding::Base64),
                        commitment: Some(CommitmentConfig::confirmed()),
                        ..Default::default()
                    },
                    with_context: None,
                    sort_results: None,
                },
            )
            .await?;
        accounts
            .into_iter()
            .map(|(address, account)| {
                let data = Round::from_bytes(&account.data)?;
                Ok(DecodedAccount {
                    address,
                    account,
                    data,
                })
            })
            .collect()
    }

    pub async fn fetch_stockpiles_by_status(
        rpc: &RpcClient,
        program_id: &Pubkey,
        status: StockpileStatus,
    ) -> Result<Vec<DecodedAccount<Stockpile>>> {
        let status_bytes = to_vec(&status)?;
        let accounts = rpc
            .get_program_accounts_with_config(
                program_id,
                RpcProgramAccountsConfig {
                    filters: Some(vec![
                        RpcFilterType::Memcmp(Memcmp::new_raw_bytes(
                            0,
                            STOCKPILE_DISCRIMINATOR.to_vec(),
                        )),
                        RpcFilterType::Memcmp(Memcmp::new_raw_bytes(8, status_bytes)),
                    ]),
                    account_config: RpcAccountInfoConfig {
                        encoding: Some(UiAccountEncoding::Base64),
                        commitment: Some(CommitmentConfig::confirmed()),
                        ..Default::default()
                    },
                    with_context: None,
                    sort_results: None,
                },
            )
            .await?;
        accounts
            .into_iter()
            .map(|(address, account)| {
                let data = Stockpile::from_bytes(&account.data)?;
                Ok(DecodedAccount {
                    address,
                    account,
                    data,
                })
            })
            .collect()
    }

    pub async fn fetch_miners_by_round(
        rpc: &RpcClient,
        program_id: &Pubkey,
        round_id: u64,
    ) -> Result<Vec<DecodedAccount<Miner>>> {
        let round_id_bytes = to_vec(&round_id)?;
        let accounts = rpc
            .get_program_accounts_with_config(
                program_id,
                RpcProgramAccountsConfig {
                    filters: Some(vec![
                        RpcFilterType::Memcmp(Memcmp::new_raw_bytes(
                            0,
                            MINER_DISCRIMINATOR.to_vec(),
                        )),
                        RpcFilterType::Memcmp(Memcmp::new_raw_bytes(8, round_id_bytes)),
                    ]),
                    account_config: RpcAccountInfoConfig {
                        encoding: Some(UiAccountEncoding::Base64),
                        commitment: Some(CommitmentConfig::confirmed()),
                        ..Default::default()
                    },
                    with_context: None,
                    sort_results: None,
                },
            )
            .await?;

        accounts
            .into_iter()
            .map(|(address, account)| {
                let data = Miner::from_bytes(&account.data)?;
                Ok(DecodedAccount {
                    address,
                    account,
                    data,
                })
            })
            .collect()
    }
}
