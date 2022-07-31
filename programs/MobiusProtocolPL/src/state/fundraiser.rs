use anchor_lang::prelude::*;

#[account]
pub struct Fundraiser {
  //fundraiser pubkey
  pub fundraiser: Pubkey,
  // amount of sol contributions 
  pub sol_qty: u64, 
  //mint of sol 
  pub sol_mint: Pubkey,
  //start of fundraising campaign
  pub start_time: u64,
  //end of fundraising campaign 
  pub end_time: u64,
  //stores sol contributions from contributors
  // pub sol_token_vault: Pubkey,
  // //soltoken vault bump 
  // pub sol_token_vault_bump: u8,  
  //fundraisers sol token account 
  pub fundraiser_sol_token_account: Pubkey,
}

impl Fundraiser {
  // leave empty
}


  // //stores sol contributions from contributors
  // pub usdc_token_vault: Pubkey,
  // //token vault bump 
  // pub usdc_token_vault_bump: u8,
  //   //fundraisers sol token account 
  //   pub fundraiser_usdc_token_account: Pubkey,

    // // amount of usdc contributions
    // pub usdc_qty: u128,
    // //mint of usdc 
    // pub usdc_mint: Pubkey,