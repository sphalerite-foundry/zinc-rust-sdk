use crate::codama_rust_custom::pda::PdaHelper;
use crate::codama_rust_custom::solana::{to_sdk_pubkey, SolanaHelper};
use anchor_lang::AccountDeserialize;
use anyhow::{anyhow, Context, Result};
use arcium_client::idl::arcium::accounts::MXEAccount;
use solana_address_lookup_table_interface::state::AddressLookupTable;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_commitment_config::CommitmentConfig;
use solana_message::AddressLookupTableAccount;
use solana_sdk::pubkey::Pubkey;

impl SolanaHelper {
    pub async fn fetch_lookup_table_account(
        rpc: &RpcClient,
        address: &Pubkey,
    ) -> Result<AddressLookupTableAccount> {
        let account = rpc
            .get_account_with_commitment(address, CommitmentConfig::confirmed())
            .await
            .map_err(|err| {
                anyhow!(
                    "failed to fetch lookup table account {address} from RPC at confirmed commitment: {err:#}"
                )
            })?
            .value
            .ok_or_else(|| anyhow!("lookup table account {address} not found at confirmed commitment"))?;
        let lookup_table = AddressLookupTable::deserialize(&account.data)
            .with_context(|| format!("failed to deserialize lookup table account {address}"))?;
        Ok(AddressLookupTableAccount {
            key: *address,
            addresses: lookup_table.addresses.to_vec(),
        })
    }

    pub async fn fetch_program_lookup_table(rpc: &RpcClient) -> Result<AddressLookupTableAccount> {
        let mxe_address = PdaHelper::get_mxe_account_address();
        let mxe_sdk_address = to_sdk_pubkey(mxe_address);
        let account = rpc
            .get_account_with_commitment(&mxe_sdk_address, CommitmentConfig::confirmed())
            .await
            .map_err(|err| {
                anyhow!(
                    "failed to fetch MXE account {mxe_address} at confirmed commitment: {err:#}"
                )
            })?
            .value
            .ok_or_else(|| {
                anyhow!("MXE account {mxe_address} not found at confirmed commitment")
            })?;
        let mut data = account.data.as_slice();
        let mxe = MXEAccount::try_deserialize(&mut data)
            .with_context(|| format!("failed to decode MXE account {mxe_address}"))?;
        let lookup_table_address = PdaHelper::get_program_lookup_table_address(mxe.lut_offset_slot);
        let lookup_table_sdk_address = to_sdk_pubkey(lookup_table_address);
        Self::fetch_lookup_table_account(rpc, &lookup_table_sdk_address).await
    }
}
