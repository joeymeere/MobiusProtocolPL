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
        payer = authority, 
        space = 50)]
        pub contributor: Account<'info, Contributor>,

        #[account(mut)]
        pub contributions: Account<'info, Contributor>,

        #[account(mut)]
        pub fundraiser: Account<'info>,

        #[account(
            mut,
            has_one = authority
        )]
        pub authority : Signer<'info>,
        pub system_program : Program<'info, System>,
        pub token_program: AccountInfo<'info>,
    }

    impl <'info> StdWithdraw <'info> {
        fn contribute(ctx:Context<StdWithdraw>, amount: u64) -> Result<()> {
            
            let time_now = Clock::get().unwrap().unix_timestamp;
    
            if time_now > fundraiser.end_time {
                return Err(error!(ErrorCode::FundraiserEnded));
            }
            else {
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
    }
    
    pub fn handler(ctx: Context<StdContribute>, amount: u64) -> Result<()> {
        //ctx.accounts.contribute(select_token, cpi_ctx, amount);
    
        Ok(())
    }
