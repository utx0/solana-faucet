use anchor_lang::prelude::*;

use super::accounts::RequestTokenAccounts;
use crate::{constant::TOKENS_PER_REQUEST, error::ProgramErrorCode};

pub fn do_request_tokens(ctx: Context<RequestTokenAccounts>) -> Result<()> {
    let clock: Clock = Clock::get().unwrap();

    let epoch_timestamp = clock.unix_timestamp;

    let mint_detail = ctx
        .accounts
        .user_meta
        .get_mint_detail(&ctx.accounts.mint.key());
    require!(
        epoch_timestamp - mint_detail.last_requested > 86_400,
        ProgramErrorCode::TokenRequestQuotaExhausted
    );

    msg!("transferring token to user");
    msg!("current unix timestamp is {}", epoch_timestamp);
    anchor_spl::token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            anchor_spl::token::Transfer {
                from: ctx.accounts.vault.to_account_info(),
                to: ctx.accounts.user_token_account.to_account_info(),
                authority: ctx.accounts.mint.to_account_info(),
            },
            &[&[
                b"hydraswap-faucet-mint".as_ref(),
                ctx.accounts.user.key().as_ref(),
                mint_detail.seed.as_bytes().as_ref(),
                &[mint_detail.bump],
            ]],
        ),
        TOKENS_PER_REQUEST,
    )?;

    ctx.accounts
        .user_meta
        .update_last_requested(&ctx.accounts.mint.key(), epoch_timestamp);

    Ok(())
}
