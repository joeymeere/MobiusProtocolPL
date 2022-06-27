use anchor_lang::prelude::*;

#[account]
pub struct Fundraiser {
  //fundraiser pubkey
  pub fundraiser: Pubkey,

  //contributor pubkey 
  pub contributor: Pubkey,
  // amount of sol contributions 
  pub sol_qty: u64,
  // amount of usdc contributions 
  pub usdc_qty: u64,

  pub usdt_qty: u64,
  //start of fundraising campaign
  pub start_time: u64,
  //end of fundraising campaign 
  pub end_time: u64,
  //stores contributions from contributors
  pub token_vault: Pubkey,
  // vault bump
  pub token_vault_bump: u8,
}

impl Fundraiser {
  // leave empty
}