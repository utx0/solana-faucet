use anchor_lang::prelude::*;

use super::accounts::SingUpAccounts;

pub fn do_user_signup(_ctx: Context<SingUpAccounts>) -> Result<()> {
    // just used for pda initialization
    Ok(())
}
