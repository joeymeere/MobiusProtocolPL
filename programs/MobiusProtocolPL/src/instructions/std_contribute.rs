use anchor_lang::prelude::*;
//use anchor_lang::solana_program::{clock, program_option::COption, sysvar};
use anchor_spl::token::{self, TokenAccount, Transfer};
use crate::state::*;

#[derive(Accounts)]
pub struct StdContribute<'info> {

    //Init contributor program PDA
    #[account(
        init,
        payer = contributor, 
        space = 8 + 32 + 8,
        constraint = Clock::get().unwrap().unix_timestamp < fundraiser_config.end_time.try_into().unwrap(),
        constraint = Clock::get().unwrap().unix_timestamp >= fundraiser_config.start_time.try_into().unwrap(),
    )]
        pub contributor_config: Account<'info, Contributor>,

        #[account(mut)]
        pub fundraiser_config: Account<'info, Fundraiser>,

        #[account(mut)]
        pub contributor_token_account: Account<'info, TokenAccount>,
        
        #[account(mut)]
        pub sol_token_vault: Box<Account<'info, TokenAccount>>,

        #[account(mut)]
        pub contributor: Signer<'info>,
        pub system_program: Program<'info, System>,
        ///CHECK: do not read or write to this program
        pub token_program: AccountInfo<'info>,
    }


impl<'info> StdContribute<'info> {
    fn update_config(&mut self, amount: u64) {
        self.fundraiser_config.sol_qty += amount;
        self.contributor_config.sol_contributions += amount;
    }

    fn transfer_to_sol_vault(&self, amount: u64) -> Result<()> {
        let sender = &self.contributor;
        let sender_of_tokens = &self.contributor_token_account;
        let recipient_of_tokens = &self.sol_token_vault;
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


pub fn handler(ctx: Context<StdContribute>, amount: u64) -> Result<()> {
    ctx.accounts.update_config(amount);
    ctx.accounts.transfer_to_sol_vault(amount);
    Ok(())
}



// fn transfer_to_usdc_vault(&mut self, amount: u128) {
//     let sender = &self.contributor;
//     let sender_of_tokens = &self.contributor_token_account;
//     let recipient_of_tokens = &self.usdc_token_vault;
//     let token_program = &self.token_program;

//     let context = Transfer {
//         from: sender_of_tokens.to_account_info(),
//         to: recipient_of_tokens.to_account_info(),
//         authority: sender.to_account_info(),
//         };

//     token::transfer(
//     CpiContext::new(token_program.to_account_info(), context),
//         amount,
//         ).expect("failed to transfer");
    
//     }