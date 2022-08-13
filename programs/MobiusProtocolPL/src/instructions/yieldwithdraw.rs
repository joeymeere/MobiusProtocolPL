use anchor_lang::prelude::*;
use anchor_spl::token::{Token, Mint};
use solend_utils;

use Utils::{solend_utils, utils};

pub struct YieldContribute {
        #[account(init,
        payer = contributor, 
        space = 50,
        constraint = Clock::get().unwrap().unix_timestamp < fundraiser_config.end_time.try_into().unwrap(),
        )]
        pub contributor_config: Box<Account<'info, Contributor>>,

        #[account(mut)]
        pub fundraiser_config: Box<Account<'info, Fundraiser>>,
        ///Source liquidity token account. [writable]
        ///                $authority can transfer $liquidity_amount.
        #[account(mut)]
        pub fundraiser: Account<'info>,
        //Destination collateral token account. [writable]
        #[account(mut)]
        pub contributor_token_account: Account<'info, TokenAccount>,
        //Reserve account. [writable]
        #[account(mut)]
        pub solend_reserve: AccountInfo<'info>,
        //Reserve liquidity supply SPL Token account. [writable]
        #[account(mut)]
        pub reserve_liqudiity: AccountInfo<'info>,
        ///Reserve collateral SPL Token mint. [writable]
        #[account(mut)]
        pub reserve_mint: Account<'info>,
        ///Lending market account.
        pub lending_market_account: AccountInfo<'info>,
        ///Derived lending market authority.
        pub lending_market_auth: AccountInfo<'info>,
        ///User transfer authority ($authority) [signer].
        #[account(mut)]
        pub contributor: Signer<'info>,
        //Clock sysvar.
        pub clock: Clock<'info>,
        //Token program id.
        pub token_program: AccountInfo<'info>,
        pub token_vault: Box<Account<'info, TokenAccount>>,
        pub collateral_token_vault: Box<Account<'info, TokenAccount>>,
        pub system_program: Program<'info, System>,
    }

impl<'info> YieldWithdraw<'info> {
  pub fn withdraw(ctx:Context<YieldWithdraw>, principal: u64) {

      let principal = pool.balance_deposited as u128;

      let cpi_ctx = CpiContext::new (redeem_reserve_collateral{
        program_id: Pubkey::from_str(DEVNET_SOLEND_PROGRAM).unwrap(),
        liquidity_amount: amount,
        source_liquidity_pubkey: contributor.to_account_info(),
        destination_collateral_pubkey: collateral_token_vault,
        reserve_pubkey: Pubkey::from_str(DEVNET_SOLEND_SOL_RESERVE).unwrap(),
        reserve_liquidity_supply_pubkey: Pubkey::from_str(DEVNET_SOLEND_CSOL_LIQUIDITY_SUPPLY).unwrap(),
        reserve_collateral_mint_pubkey: Pubkey::from_str(DEVNET_SOLEND_CSOL_COLLATERAL_MINT).unwrap(),
        lending_market_pubkey: Pubkey::from_str(DEVNET_SOLEND_LENDING_MARKET).unwrap(),
        user_transfer_authority_pubkey: contributor.to_account_info(),
      }
    );
      solend_utils::devnet_solend_redeem_reserve_collateral(cpi_ctx, principal);

      let cpi_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        token::Transfer {
          from: ctx.accounts.token_vault.to_account_info(),
          to: ctx.accounts.contributor.to_account.info(),
          authority: ctx.accounts.token_vault.to_account_info(),
        }
      );
        token::transfer(cpi_ctx, principal);
      }
    }

    pub fn handler(ctx: Context<StdContribute>, amount: u64, select_token: u8) {
    
        ctx.accounts.withdraw(ctx, cpi_ctx, principal);
    
    }