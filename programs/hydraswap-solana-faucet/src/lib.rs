mod constant;
mod error;
mod ix;
mod state;

use anchor_lang::prelude::*;
use ix::a_create_mint_and_vault::accounts::*;
use ix::b_request_tokens::accounts::*;

declare_id!("3XaCom5sYWVs5RRkaZg2iudZgnKFhnYt7dbXau8puUvW");

#[program]
pub mod hydraswap_solana_faucet {
    use crate::ix::{
        a_create_mint_and_vault::ix::do_create_mint_and_vault,
        b_request_tokens::ix::do_request_tokens,
    };

    use super::*;

    pub fn create_mint_and_vault(
        ctx: Context<CreateMintAndVaultAccounts>,
        faucet_seed: String,
        mint_amount: u64,
    ) -> Result<()> {
        do_create_mint_and_vault(ctx, faucet_seed, mint_amount)
    }

    pub fn request_tokens(ctx: Context<RequestTokenAccounts>) -> Result<()> {
        do_request_tokens(ctx)
    }
}
