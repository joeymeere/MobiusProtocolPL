use anchor_lang::prelude::*;

#[account]
pub struct Fundraiser {
  //token vault 
  pub token_vault: Pubkey,
  //test sol mint 
  pub sol_mint: Pubkey,
  //fundraiser pubkey
  pub fundraiser: Pubkey,
  // amount of sol contributions 
  pub sol_qty: u64, 
  //fundraiser token account for withdrawing to 
  pub fundraiser_token_account: Pubkey,
  //target amount to be raised 
  pub goal: u64,

  pub name: String,

  pub description: String,

  pub image_link: String,

  pub website_link: String,

  pub contact_link: String,

}