use anchor_lang::prelude::*;

use crate::state::*;
use anchor_spl::token::{self, Mint, SetAuthority, TokenAccount, Transfer};
use spl_token::instruction::AuthorityType;

#[derive(Accounts)]
pub struct CreateCampaign<'info> {
  // define accounts taken in by the CreateGame instruction
  #[account(init, payer = host, space = 8 + (32 * 4) + 16 + (8 * 6) + (1 * 3))]
  pub game_config: Account<'info, Game>,

  #[account(mut)]
  pub host: Signer<'info>,

  #[account(mut, constraint = host_reward_account.mint == reward_mint.key())]
  pub host_reward_account: Account<'info, TokenAccount>,

  #[account(mut)]
  pub reward_mint: Account<'info, Mint>,

  #[account(
      init,
      seeds = [b"reward-escrow".as_ref(), game_config.to_account_info().key.as_ref()],
      bump,
      payer = host,
      token::mint = reward_mint,
      token::authority = host,
  )]
  pub reward_escrow: Account<'info, TokenAccount>,

  pub system_program: Program<'info, System>,
  pub rent: Sysvar<'info, Rent>,
  /// CHECK: This is not dangerous because we don't read or write from this account
  pub token_program: AccountInfo<'info>,
}

impl<'info> CreateGame<'info> {
  // implement required functions for CreateGame struct

  //not quite sure on this portion on how to set it up.. any pointers will be great!
  fn set_game_config(
    &mut self,
    join: u64, 
    start: u64, 
    end: u64, 
    start_usd: u64, 
    winners: u8, 
    max_players: u64,
    reward_amount: u64,
    reward_escrow_bump: u8,
  ) {
    self.game_config.host = *self.host.to_account_info().key;
    self.game_config.host_reward_account = *self.host_reward_account.to_account_info().key;
    self.game_config.reward_amount = reward_amount;
    self.game_config.join_time = join;
    self.game_config.start_time = start;
    self.game_config.end_time = end;
    self.game_config.start_usd = start_usd as u128;
    self.game_config.current_cap = 0;
    self.game_config.max_cap = max_players;
    self.game_config.winners = winners;
    self.game_config.reward_mint = *self.reward_mint.to_account_info().key;
    self.game_config.reward_escrow = *self.reward_escrow.to_account_info().key;
    self.game_config.reward_escrow_bump = reward_escrow_bump;
    self.game_config.game_ended = false;
  }

  fn set_authority_escrow(&self, program_id: &anchor_lang::prelude::Pubkey) {
    const ESCROW_PDA_SEED: &[u8] = b"authority-seed";
    let (vault_authority, _vault_authority_bump) = Pubkey::find_program_address(
      &[
        ESCROW_PDA_SEED,
        self.game_config.to_account_info().key.as_ref(),
      ],
      program_id,
    );
    let cpi_accounts = SetAuthority {
      account_or_mint: self.reward_escrow.to_account_info().clone(),
      current_authority: self.host.to_account_info().clone(),
    };
    token::set_authority(
      CpiContext::new(self.token_program.clone(), cpi_accounts),
      AuthorityType::AccountOwner,
      Some(vault_authority),
    )
    .unwrap();
  }

  fn transfer_host_reward(&self, amount: u64) -> Result<()> {
    let sender = &self.host;
    let sender_of_tokens = &self.host_reward_account;
    let recipient_of_tokens = &self.reward_escrow;
    let token_program = &self.token_program;

    let context = Transfer {
      from: sender_of_tokens.to_account_info(),
      to: recipient_of_tokens.to_account_info(),
      authority: sender.to_account_info(),
    };

    token::transfer(
      CpiContext::new(token_program.to_account_info(), context),
      amount,
    )
  }
}

pub fn handler(
  ctx: Context<CreateGame>, 
  join: u64, 
  start: u64, 
  end: u64, 
  start_usd: u64, 
  winners: u8, 
  max_players: u64,
  reward_amount: u64,
  reward_escrow_bump: u8,
) -> Result<()> {
  // core instruction to allow hosts to create a game account
  // must pass in required settings (join, start, end, rewards, etc) to game account
  ctx.accounts.set_game_config(
    join, 
    start, 
    end, 
    start_usd,
    winners, 
    max_players,
    reward_amount,
    reward_escrow_bump,
  );

  ctx.accounts.set_authority_escrow(ctx.program_id);
  ctx.accounts.transfer_host_reward(reward_amount)
}