use anchor_lang::prelude::*;

#[account]
pub struct Fundraiser {
  //fundraiser pubkey
  pub fundraiser: Pubkey,
  //contributor pubkey 
  pub contributor: Pubkey,
  // amount of sol contributions 
  pub sol_qty: u128, 
  //mint of sol 
  pub sol_mint: Pubkey,
  // amount of usdc contributions
  pub usdc_qty: u128,
  //mint of usdc 
  pub usdc_mint: Pubkey,
  //start of fundraising campaign
  pub start_time: u64,
  //end of fundraising campaign 
  pub end_time: u64,
  //stores sol contributions from contributors
  pub sol_token_vault: Pubkey,
  //token vault bump 
  pub sol_token_vault_bump: u8,
  //stores sol contributions from contributors
  pub usdc_token_vault: Pubkey,
  //token vault bump 
  pub usdc_token_vault_bump: u8,  
  //fundraisers sol token account 
  pub fundraiser_sol_token_account: Pubkey,
  // fundraiser sol token account bump
  pub fundraiser_sol_bump: u8,
  //fundraisers sol token account 
  pub fundraiser_usdc_token_account: Pubkey,
  // fundraiser sol token account bump
  pub fundraiser_usdc_bump: u8,
}

impl Fundraiser {
  // leave empty
}