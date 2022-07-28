use anchor_lang::prelude::*;

#[account]
pub struct Contributor {
  //contributor pubkey
  pub contributor: Pubkey,
  // sol qty 
  pub sol_contributions: u128,
  // usdc qty 
  pub usdc_contributions: u128,
  //contributor config
  pub fundraiser_config: Pubkey,
}

impl Contributor {
  // leave empty
}
