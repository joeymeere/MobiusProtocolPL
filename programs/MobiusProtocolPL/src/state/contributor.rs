use anchor_lang::prelude::*;

#[account]
pub struct Contributor {
  //contributor pubkey
  pub contributor: Pubkey,
  //contributions
  pub contributions: u128,
  //fund bump
  pub contributor_bump: u8,
  //game pubkey,
  pub fundraiser: Pubkey,
}

impl Contributor {
  // leave empty
}
