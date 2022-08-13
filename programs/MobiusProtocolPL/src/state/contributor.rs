use anchor_lang::prelude::*;
use solana_program::sysvar::sysvar;
use anchor_spl::token::Mint;

#[account]
pub struct Contributor {

  pub contributor: Pubkey,

  pub contributions: u128,

  pub contributor_bump: u8,

  pub fundraiser: Pubkey,

  pub contributor_config: Pubkey,

  pub contributor_token_account: Pubkey,

  pub solend_reserve: Pubkey,

  pub reserve_liqudiity: Pubkey, 

  pub reserve_mint: Mint,

  pub lending_market_account: Pubkey,

  pub collateral_token_vault: Pubkey,

  pub lending_market_auth: Pubkey,

  pub clock: Sysvar, 

  pub token_program: Pubkey,

  pub token_vault: Pubkey,

  pub system_program: Pubkey,
}

impl Contributor {
  // leave empty
}
