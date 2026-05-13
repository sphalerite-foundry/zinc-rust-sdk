# Zinc Rust SDK

Rust SDK for the Zinc protocol.

This crate packages the generated Codama Rust client and handwritten Zinc PDA,
instruction, and Solana helper wrappers.

## Publication Posture

This repository is intended for source-visible public review from a clean
reviewed import. Existing private history should not be made public unless a
separate history scan and human approval explicitly allow it.

The crate is rights reserved and keeps `license = "UNLICENSED"` and
`publish = false`; crates.io publishing is out of scope for the initial public
source release.

## Environment Variables

The SDK only reads optional compute-unit price configuration when transaction
helpers are used:

- `ZINC_URGENT_COMPUTE_UNIT_PRICE_MICRO_LAMPORTS`
- `ZINC_BACKGROUND_COMPUTE_UNIT_PRICE_MICRO_LAMPORTS`
- `ZINC_COMPUTE_UNIT_PRICE_MICRO_LAMPORTS`

Do not commit `.env` files, RPC URLs containing API keys, keypair JSON, wallet
files, private keys, or seed phrases to this repository.
