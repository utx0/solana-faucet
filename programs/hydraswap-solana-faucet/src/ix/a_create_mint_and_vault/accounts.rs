use crate::state::user_meta::UserMeta;
use anchor_lang::prelude::*;
use anchor_lang::system_program;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::Mint;
use anchor_spl::token::Token;
use std::string::String;

#[derive(Accounts)]
#[instruction(faucet_seed: String)]
pub struct CreateMintAndVaultAccounts<'info> {
    // user meta account
    #[account(
        init_if_needed,
        space= 4 + (32 + 5 + 4 * 15 + 8 + 1 ) * 2 ,
        seeds=[b"user_meta_v0".as_ref(), user.key().as_ref()],
        bump,
        payer=user
    )]
    pub user_meta: Account<'info, UserMeta>,

    #[account(mut)]
    pub user: Signer<'info>,

    // mint to be created
    /// CHECK: not necessary as we expect this to be uninitialized account
    #[account(
        init,
        payer = user,
        seeds = [
            b"hydraswap-faucet-mint".as_ref(),
            user.key().as_ref(),
            faucet_seed.as_bytes().as_ref()],
        bump,
        mint::decimals = 6,
        mint::authority = mint
    )]
    pub mint: Account<'info, Mint>,

    // vault address instead of creating it here
    // and updating the authority later have done it in
    // the instruction itself
    /// CHECK: it's okay to pass it as is
    #[account(mut)]
    pub vault: AccountInfo<'info>,

    /// token program
    // #[account(address = Token::id())]
    pub token_program: Program<'info, Token>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,

    /// associated token account prog
    pub associated_token_program: Program<'info, AssociatedToken>,

    pub rent: Sysvar<'info, Rent>,
}
