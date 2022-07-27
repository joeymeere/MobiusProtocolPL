use anchor_lang::prelude::*;

#[account]
pub struct Contributor {
  //contributor pubkey
  pub contributor: Pubkey,
  //contributor config
  pub contributor_config: Pubkey,
  //contributions
  pub contributions: u128,
  //game pubkey,
  pub fundraiser: Pubkey,
}

impl Contributor {
  // leave empty
}
