use anchor_lang::prelude::*;
use anchor_lang::solana_program::{clock, program_option::COption, sysvar};
use anchor_spl::token::{self, Token, Mint, TokenAccount, Transfer};
use crate::state::*;
use crate::create_fundraiser::*;

#[derive(Accounts)]
pub struct StdContribute<'info> {

    //Init contributor program PDA
    #[account(init,
        payer = contributor, 
        space = 50,
        constraint = Clock::get().unwrap().unix_timestamp < fundraiser_config.end_time.try_into().unwrap(),
        )]
        pub contributor_config: Account<'info, Contributor>,

        #[account(mut)]
        pub fundraiser_config: Account<'info, Fundraiser>,

        #[account(mut)]
        pub contributor_token_account: Account<'info, TokenAccount>,
        pub token_vault: Account<'info, TokenAccount>,

        #[account(mut)]
        pub contributor: Signer<'info>,
        pub system_program: Program<'info, System>,
        pub token_program: AccountInfo<'info>,
    }


impl <'info> StdContribute<'info> {
    fn contribute(&mut self, amount: u64, select_token: u8) -> Result<()> {
        match select_token {
            1 => {
                self.fundraiser_config.sol_qty += amount;
                Ok(self.fundraiser_config.sol_qty)
                },

            2 => {
                self.fundraiser_config.usdc_qty += amount;
                Ok(self.fundraiser_config.usdc_qty)
                },

            3 => {
                self.fundraiser_config.usdt_qty += amount;
                Ok(self.fundraiser_config.usdt_qty)
                },
        }.unwrap();
    
        /* 
            let cpi_ctx = CpiContext::new(
                self.token_program.to_account_info(),
                token::Transfer {
                    from: self.contributor.to_account_info(),
                    to: self.fundraiser_config.token_vault.to_account_info(),
                    authority: self.contributor.to_account_info(), 
                },
              
            );
            token::transfer(cpi_ctx, amount)?;
        */

    fn transfer_contribution(&self, amount: u64) -> Result<()> {
                let sender = &self.contributor;
                let sender_of_tokens = &self.contributor_token_account;
                let recipient_of_tokens = &self.token_vault;
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
           Ok(())
            }
    }


pub fn handler(ctx: Context<StdContribute>, amount: u64, select_token: u8) -> Result<()> {
    ctx.accounts.contribute(amount, select_token);

    Ok(())
}