use anchor_lang::prelude::*;

use crate::state::*;
use anchor_spl::token::{self, Mint, TokenAccount, SetAuthority};
use spl_token::instruction::AuthorityType;

#[derive(Accounts)]
pub struct CreateCampaign<'info> {
  // discriminator + pubkey * 7 + u128 * 2 + u64 * 2 + U8 * 2
  #[account(init, payer = fundraiser, space = 8 + (32 * 7) + (16 * 2) + (8 * 2) + (2 * 1))]
  pub fundraiser_config: Box<Account<'info, Fundraiser>>,

  #[account(mut)]
  pub fundraiser: Signer<'info>,

  #[account(mut)]
  pub fundraiser_sol_token_account: Box<Account<'info, TokenAccount>>,

  #[account(mut)]
  pub fundraiser_usdc_token_account: Box<Account<'info, TokenAccount>>,

  #[account(
      init,
      seeds = [b"sol-token-vault".as_ref(), fundraiser_config.to_account_info().key.as_ref()],
      bump,
      payer = fundraiser,
      token::authority = fundraiser_config,
      token::mint = sol_mint
  )]
  pub sol_token_vault: Box<Account<'info, TokenAccount>>,

  #[account(mut)]
  pub sol_mint: Box<Account<'info, Mint>>,

  #[account(
    init,
    seeds = [b"usdc-token-vault".as_ref(), fundraiser_config.to_account_info().key.as_ref()],
    bump,
    payer = fundraiser,
    token::authority = fundraiser_config,
    token::mint = usdc_mint
  )]
  pub usdc_token_vault: Box<Account<'info, TokenAccount>>,

  #[account(mut)]
  pub usdc_mint: Box<Account<'info, Mint>>,

  pub system_program: Program<'info, System>,
  pub rent: Sysvar<'info, Rent>,
  /// CHECK: This is not dangerous because we don't read or write from this account
  pub token_program: AccountInfo<'info>,
}

impl<'info> CreateCampaign<'info> {
  // implement required functions for CreateGame struct

  //not quite sure on this portion on how to set it up.. any pointers will be great!
  fn set_fundraiser_config(
    &mut self, 
    start: u64, 
    end: u64, 
    sol_token_vault_bump: u8,
    usdc_token_vault_bump: u8,
  ) {
    self.fundraiser_config.fundraiser = *self.fundraiser.to_account_info().key;
    self.fundraiser_config.start_time = start;
    self.fundraiser_config.end_time = end;
    self.fundraiser_config.sol_qty = 0;
    self.fundraiser_config.sol_mint = *self.sol_mint.to_account_info().key;
    self.fundraiser_config.usdc_qty = 0;
    self.fundraiser_config.usdc_mint = *self.usdc_mint.to_account_info().key;
    self.fundraiser_config.sol_token_vault = *self.sol_token_vault.to_account_info().key;
    self.fundraiser_config.sol_token_vault_bump = sol_token_vault_bump;
    self.fundraiser_config.usdc_token_vault = *self.usdc_token_vault.to_account_info().key;
    self.fundraiser_config.usdc_token_vault_bump = usdc_token_vault_bump;
  }

  fn set_authority_sol_token_vault(&self, program_id: &anchor_lang::prelude::Pubkey) {
    const ESCROW_PDA_SEED: &[u8] = b"authority-seed";
    let (vault_authority, _vault_authority_bump) = Pubkey::find_program_address(
      &[
        ESCROW_PDA_SEED,
        self.fundraiser_config.to_account_info().key.as_ref(),
      ],
      program_id,
    );
    let cpi_accounts = SetAuthority {
      account_or_mint: self.sol_token_vault.to_account_info().clone(),
      current_authority: self.fundraiser_config.to_account_info().clone(),
    };
    token::set_authority(
      CpiContext::new(self.token_program.clone(), cpi_accounts),
      AuthorityType::AccountOwner,
      Some(vault_authority),
    )
    .unwrap();
  }

  fn set_authority_usdc_token_vault(&self, program_id: &anchor_lang::prelude::Pubkey) {
    const ESCROW_PDA_SEED: &[u8] = b"authority-seed";
    let (vault_authority, _vault_authority_bump) = Pubkey::find_program_address(
      &[
        ESCROW_PDA_SEED,
        self.fundraiser_config.to_account_info().key.as_ref(),
      ],
      program_id,
    );
    let cpi_accounts = SetAuthority {
      account_or_mint: self.usdc_token_vault.to_account_info().clone(),
      current_authority: self.fundraiser_config.to_account_info().clone(),
    };
    token::set_authority(
      CpiContext::new(self.token_program.clone(), cpi_accounts),
      AuthorityType::AccountOwner,
      Some(vault_authority),
    )
    .unwrap();
  }
}

pub fn handler(
  ctx: Context<CreateCampaign>, 
  start: u64, 
  end: u64, 
  sol_token_vault_bump: u8,
  usdc_token_vault_bump: u8,
) -> Result<()> {
  // core instruction to allow hosts to create a game account
  // must pass in required settings (join, start, end, rewards, etc) to game account
  ctx.accounts.set_fundraiser_config(
    start, 
    end, 
    sol_token_vault_bump,
    usdc_token_vault_bump,
  );
  ctx.accounts.set_authority_sol_token_vault(ctx.program_id);
  ctx.accounts.set_authority_usdc_token_vault(ctx.program_id);
  Ok(())
}
