use anchor_lang::prelude::*;

#[account]
pub struct Fundraiser {
  //token vault 
  pub token_vault: Pubkey,
  //token vault mint 
  pub sol_mint: Pubkey,
  //fundraiser pubkey
  pub fundraiser: Pubkey,
  // amount of sol contributions 
  pub sol_qty: u64, 
  //fundraiser token account for withdrawing to 
  pub fundraiser_sol_token_account: Pubkey,
  //target amount to be raised 
  pub goal: u64
}