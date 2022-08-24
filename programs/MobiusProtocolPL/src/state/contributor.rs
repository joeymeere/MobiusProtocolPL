use anchor_lang::prelude::*;

#[account]
pub struct Contributor {
  //which fundraiser 
  pub fundraiser_config: Pubkey,
  //contributor pubkey
  pub contributor: Pubkey,
  // sol qty 
  pub sol_contributions: u64,
}

