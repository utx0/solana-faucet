use std::collections::BTreeMap;

use anchor_lang::prelude::*;

#[account]
pub struct UserMeta {
    // instead of maintaining additional field
    // for counting will depend on option itself
    pub mint_details: BTreeMap<Pubkey, MintDetails>,
}

//
impl UserMeta {
    /// can current user mint more?
    pub fn can_mint_more(&self) -> bool {
        self.mint_details.len() < 2
    }

    pub fn get_mint_detail(&self, mint: &Pubkey) -> MintDetails {
        self.mint_details.get(mint).unwrap().clone()
    }

    pub fn update_last_requested(&mut self, mint: &Pubkey, last_requested: i64) {
        self.mint_details.get_mut(mint).unwrap().last_requested = last_requested;
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct MintDetails {
    pub seed: String,
    pub bump: u8,
    pub last_requested: i64,
}
