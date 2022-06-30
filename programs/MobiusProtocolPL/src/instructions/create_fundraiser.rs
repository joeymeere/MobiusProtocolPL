use anchor_lang::prelude::*;

use crate::state::*;
use anchor_spl::token::{self, TokenAccount, SetAuthority};
use spl_token::instruction::AuthorityType;

#[derive(Accounts)]
pub struct CreateCampaign<'info> {
  // define accounts taken in by the CreateGame instruction
  #[account(init, payer = fundraiser, space = 8 + (32 * 3) + (16 * 2) + (8 * 2) + 1)]
  pub fundraiser_config: Account<'info, Fundraiser>,

  #[account(mut)]
  pub fundraiser: Signer<'info>,

  #[account(
      init,
      seeds = [b"token-vault".as_ref(), fundraiser_config.to_account_info().key.as_ref()],
      bump,
      space = 9999,
      payer = fundraiser,
  )]
  pub token_vault: Account<'info, TokenAccount>,

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
    reward_escrow_bump: u8,
  ) {
    self.fundraiser_config.fundraiser = *self.fundraiser.to_account_info().key;
    self.fundraiser_config.start_time = start;
    self.fundraiser_config.end_time = end;
    self.fundraiser_config.sol_qty = 0;
    self.fundraiser_config.usdc_qty = 0;
    self.fundraiser_config.token_vault = *self.token_vault.to_account_info().key;
    self.fundraiser_config.token_vault_bump = reward_escrow_bump;
  }

  fn set_authority_token_vault(&self, program_id: &anchor_lang::prelude::Pubkey) {
    const ESCROW_PDA_SEED: &[u8] = b"authority-seed";
    let (vault_authority, _vault_authority_bump) = Pubkey::find_program_address(
      &[
        ESCROW_PDA_SEED,
        self.fundraiser_config.to_account_info().key.as_ref(),
      ],
      program_id,
    );
    let cpi_accounts = SetAuthority {
      account_or_mint: self.token_vault.to_account_info().clone(),
      current_authority: self.fundraiser.to_account_info().clone(),
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
  reward_escrow_bump: u8,
) -> Result<()> {
  // core instruction to allow hosts to create a game account
  // must pass in required settings (join, start, end, rewards, etc) to game account
  ctx.accounts.set_fundraiser_config(
    start, 
    end, 
    reward_escrow_bump,
  );
  ctx.accounts.set_authority_token_vault(ctx.program_id);
  Ok(())
}
