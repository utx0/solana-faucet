use anchor_lang::prelude::*;

use super::accounts::CreateMintAndVaultAccounts;
use crate::{error::ProgramErrorCode, state::user_meta::MintDetails};

pub fn do_create_mint_and_vault(
    ctx: Context<CreateMintAndVaultAccounts>,
    faucet_seed: String,
    mint_amount: u64,
) -> Result<()> {
    // check if the minting quota is not exhausted
    require!(
        ctx.accounts.user_meta.can_mint_more(),
        ProgramErrorCode::MintingQuotaExhausted
    );
    msg!("more token can be minted");

    let mint_bump = ctx.bumps.get("mint").unwrap().to_owned();

    msg!("firing associated token create");
    anchor_spl::associated_token::create(CpiContext::new_with_signer(
        ctx.accounts.associated_token_program.to_account_info(),
        anchor_spl::associated_token::Create {
            payer: ctx.accounts.user.to_account_info(),
            associated_token: ctx.accounts.vault.to_account_info(),
            authority: ctx.accounts.mint.to_account_info(),
            mint: ctx.accounts.mint.to_account_info(),
            rent: ctx.accounts.rent.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
        },
        &[&[
            b"hydraswap-faucet-mint".as_ref(),
            ctx.accounts.user.key().as_ref(),
            faucet_seed.as_bytes().as_ref(),
            &[mint_bump],
        ]],
    ))?;

    msg!("firing mint to ");
    anchor_spl::token::mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            anchor_spl::token::MintTo {
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.vault.to_account_info(),
                authority: ctx.accounts.mint.to_account_info(),
            },
            &[&[
                b"hydraswap-faucet-mint".as_ref(),
                ctx.accounts.user.key().as_ref(),
                faucet_seed.as_bytes().as_ref(),
                &[mint_bump],
            ]],
        ),
        mint_amount,
    )?;

    let mint_detail = MintDetails {
        mint: ctx.accounts.mint.key(),
        last_requested: 0,
        seed: faucet_seed,
        bump: mint_bump,
    };

    ctx.accounts.user_meta.mint_details.push(mint_detail);
    Ok(())
}
