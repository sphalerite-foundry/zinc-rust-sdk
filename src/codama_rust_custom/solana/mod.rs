pub mod account;
pub mod lut;
pub mod transaction;

pub struct SolanaHelper;

pub(crate) fn to_sdk_pubkey(address: solana_address::Address) -> solana_sdk::pubkey::Pubkey {
    solana_sdk::pubkey::Pubkey::new_from_array(address.to_bytes())
}
