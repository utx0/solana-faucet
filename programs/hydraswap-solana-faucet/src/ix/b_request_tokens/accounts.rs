use crate::state::user_meta::UserMeta;
use anchor_lang::prelude::*;
use anchor_lang::system_program;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::Mint;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;
use std::string::String;

#[derive(Accounts)]
pub struct RequestTokenAccounts<'info> {
    /// bounty account
    #[account(mut)]
    pub user_meta: Account<'info, UserMeta>,

    #[account(mut)]
    pub user: Signer<'info>,

    // mint
    pub mint: Account<'info, Mint>,

    // vault account
    #[account(mut)]
    pub vault: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = mint,
        associated_token::authority = user
    )]
    pub user_token_account: Account<'info, TokenAccount>,

    /// token program
    pub token_program: Program<'info, Token>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,

    /// associated token account prog
    pub associated_token_program: Program<'info, AssociatedToken>,

    pub rent: Sysvar<'info, Rent>,
}
