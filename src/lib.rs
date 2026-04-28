#![allow(
    dead_code,
    unexpected_cfgs,
    unused_imports,
    unused_parens,
    clippy::all,
    clippy::pedantic,
    clippy::allow_attributes,
    clippy::allow_attributes_without_reason,
    clippy::unwrap_used,
    clippy::expect_used,
    reason = "generated Codama bindings are exempt from handwritten Clippy policy"
)]

#[path = "codama_rust/accounts/mod.rs"]
pub mod accounts;
#[path = "codama_rust/errors/mod.rs"]
pub mod errors;
#[path = "codama_rust/instructions/mod.rs"]
pub mod instructions;
#[path = "codama_rust/programs.rs"]
pub mod programs;
#[path = "codama_rust/shared.rs"]
pub mod shared;
#[path = "codama_rust/types/mod.rs"]
pub mod types;

/// Compatibility namespace for callers that imported the copied Codama tree.
pub mod codama_rust {
    pub use crate::accounts;
    pub use crate::errors;
    pub use crate::instructions;
    pub use crate::programs;
    pub use crate::programs::ZINC_ID;
    pub use crate::shared;
    pub use crate::types;
}

pub mod codama_rust_custom;

/// Compatibility namespace for callers that imported raw Codama output.
pub mod generated {
    pub use crate::codama_rust::*;
}

pub use codama_rust::*;
pub use programs::ZINC_ID;
