//! ### Errors
//!
//! all possible errors which could occur during the
//! solana program execution
//!
use anchor_lang::prelude::*;

/// ErrorCode enum\
///
#[error_code]
pub enum ProgramErrorCode {
    #[msg("Your minting quota exhausted (2/2)")]
    MintingQuotaExhausted,
    #[msg("Token request Quota for the day has been exhausted (1/1)")]
    TokenRequestQuotaExhausted,
}
