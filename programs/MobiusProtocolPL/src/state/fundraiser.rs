use anchor_lang::prelude::*;

#[account]
pub struct Fundraiser {
  //fundraiser pubkey
  pub fundraiser: Pubkey,
  //contributor pubkey 
  pub contributor: Pubkey,
  //no. of contributors for fundraising project 
  pub contributors: u128,
  // amount of contributions 
  pub contributions: u128,
  //start of fundraising campaign
  pub start_time: u64,
  //end of fundraising campaign 
  pub end_time: u64,
  //stores contributions from contributors
  pub token_vault: Pubkey,
  // vault bump
  pub token_vault_bump: u8,
}

impl Game {
  // leave empty
}