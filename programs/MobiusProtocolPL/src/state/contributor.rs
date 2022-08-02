use anchor_lang::prelude::*;

#[account]
pub struct Contributor {
  //contributor pubkey
  pub contributor: Pubkey,
  // sol qty 
  pub sol_contributions: u64,
}

impl Contributor {
  // leave empty
}
