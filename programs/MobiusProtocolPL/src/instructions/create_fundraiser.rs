use anchor_lang::prelude::*;

use crate::state::*;
use anchor_spl::token::{self, TokenAccount, SetAuthority};
use spl_token::instruction::AuthorityType;

#[derive(Accounts)]
pub struct CreateCampaign<'info> {
  // discriminator + pubkey * 4 + u128 * 2 + u64 * 2 + U8 * 2
  #[account(init, payer = fundraiser, space = 8 + (32 * 4) + 16 + 1)]
  pub fundraiser_config: Box<Account<'info, Fundraiser>>,

  #[account(mut)]
  pub fundraiser: Signer<'info>,  

  #[account(mut)]
  pub fundraiser_sol_token_account: Box<Account<'info, TokenAccount>>,

  #[account(mut)]
  pub sol_token_vault: Box<Account<'info, TokenAccount>>,

  pub system_program: Program<'info, System>,
  pub rent: Sysvar<'info, Rent>,
  /// CHECK: This is not dangerous because we don't read or write from this account
  pub token_program: AccountInfo<'info>,
}

impl<'info> CreateCampaign<'info> {
  // implement required functions for CreateGame struct
  fn set_fundraiser_config(
    &mut self, 
  ) {
    self.fundraiser_config.fundraiser = *self.fundraiser.to_account_info().key;
    self.fundraiser_config.sol_qty = 0;
    // self.fundraiser_config.sol_token_vault = *self.sol_token_vault.to_account_info().key;
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
}

pub fn handler(
  ctx: Context<CreateCampaign>, 
) -> Result<()> {
  // core instruction to allow hosts to create a game account
  ctx.accounts.set_fundraiser_config();
  ctx.accounts.set_authority_sol_token_vault(ctx.program_id);
  Ok(())
}





// self.fundraiser_config.sol_token_vault_bump = sol_token_vault_bump;
// self.fundraiser_config.usdc_token_vault = *self.usdc_token_vault.to_account_info().key;
// // self.fundraiser_config.usdc_token_vault_bump = usdc_token_vault_bump;
// self.fundraiser_config.usdc_qty = 0;
// self.fundraiser_config.usdc_mint = *self.usdc_mint.to_account_info().key;
// self.fundraiser_config.sol_mint = *self.sol_mint.to_account_info().key;

// #[account(
//   init,
//   seeds = [b"usdc-token-vault".as_ref(), fundraiser_config.to_account_info().key.as_ref()],
//   bump,
//   payer = fundraiser,
//   token::authority = fundraiser_config,
//   token::mint = usdc_mint
// )]
// pub usdc_token_vault: Box<Account<'info, TokenAccount>>,

// #[account(mut)]
// pub usdc_mint: Box<Account<'info, Mint>>,


// fn set_authority_usdc_token_vault(&self, program_id: &anchor_lang::prelude::Pubkey) {
//   const ESCROW_PDA_SEED: &[u8] = b"authority-seed";
//   let (vault_authority, _vault_authority_bump) = Pubkey::find_program_address(
//     &[
//       ESCROW_PDA_SEED,
//       self.fundraiser_config.to_account_info().key.as_ref(),
//     ],
//     program_id,
//   );
//   let cpi_accounts = SetAuthority {
//     account_or_mint: self.usdc_token_vault.to_account_info().clone(),
//     current_authority: self.fundraiser_config.to_account_info().clone(),
//   };
//   token::set_authority(
//     CpiContext::new(self.token_program.clone(), cpi_accounts),
//     AuthorityType::AccountOwner,
//     Some(vault_authority),
//   )
//   .unwrap();
// }