use anchor_lang::prelude::*;
use anchor_lang::system_program;

use crate::state::user_meta::UserMeta;

#[derive(Accounts)]
pub struct SingUpAccounts<'info> {
    /// user info meta
    #[account(
        init,
        space= 4 + (32 + 5 + 4 * 15 + 8 + 1 ) * 2 ,
        seeds=[b"user_meta_v0".as_ref(), user.key().as_ref()],
        bump,
        payer=user
    )]
    pub user_meta: Account<'info, UserMeta>,

    #[account(mut)]
    pub user: Signer<'info>,

    /// system program
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}
