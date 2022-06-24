use anchor_lang::prelude::*;
use anchor_lang::solana_program::{clock, program_option::COption, sysvar};
use anchor_spl::token::{self, Token, Mint, TokenAccount, Transfer};
use crate::state::*;
use create_fundraiser::*;

#[derive(Accounts)]
pub struct StdWithdraw {

    //Init contributor program PDA
    #[account(mut, 
        init, 
        seeds = [b"ILikeTurtles!".as_ref(), authority.key().as_ref()], 
        bump = contributor_account_bump, 
        payer = contributor, 
        space = 50
        constraint = Clock::get().unwrap().unix_timestamp < fundraiser_config.end_time.try_into().unwrap(), 
        )]
        pub contributor_config: Account<'info, Contributor>,

        #[account(mut)]
        pub contributions: Account<'info, Contributor>,

        #[account(mut)]
        pub fundraiser: Account<'info>,

        #[account(mut)]
        pub contributor : Signer<'info>,
        pub system_program : Program<'info, System>,
        pub token_program: AccountInfo<'info>,
    }


impl <'info> StdWithdraw <'info> {
    fn contribute(ctx:Context<StdWithdraw>, amount: u64) -> Result<()> {
        match select_token {
            1 => {
                self.fundraiser.sol_qty -= amount;
                Ok(self.fundraiser.sol_qty)
                },

            2 => {
                self.fundraiser.usdc_qty -= amount;
                Ok(self.fundraiser.usdc_qty)
                },

            3 => {
                self.fundraiser.usdt_qty -= amount;
                Ok(self.fundraiser.usdt_qty)
                },
        }.unwrap();

            let cpi_ctx = CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::Transfer {
                    from: ctx.accounts.fundraiser.token_vault.to_account_info(),
                    to: ctx.accounts.contributor.to_account_info(),
                    authority: ctx.accounts.contributor.to_account_info(), 
                },
            );
            
            token::transfer(cpi_ctx, amount)?;
            }
        }


pub fn handler(ctx: Context<StdWithdraw>, amount: u64) -> Result<()> {
    ctx.accounts.contribute(select_token, cpi_ctx, amount);

    Ok(())
}