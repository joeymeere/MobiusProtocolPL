use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, Mint, TokenAccount, Transfer};
use crate::state::*;

#[derive(Accounts)]
pub struct FundraiserWithdrawal<'info> {

    #[account(mut)]
    pub vault_pda: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub fundraiser_config: Box<Account<'info, Fundraiser>>,
    
    #[account(mut)]
    pub sol_token_vault: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub fundraiser_sol_token_account: Box<Account<'info, TokenAccount>>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    ///CHECK: do not read or write to this program
    pub token_program: AccountInfo<'info>,
    }


impl<'info> FundraiserWithdrawal<'info> {
    fn update_config(&mut self, amount: u64) {
        self.fundraiser_config.sol_qty -= amount;
    }

    fn transfer_to_fundraiser(&self, amount: u64) -> Result<()> {
        let sender = &self.vault_pda;
        let sender_of_tokens = &self.sol_token_vault;
        let recipient_of_tokens = &self.fundraiser_sol_token_account;
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


pub fn handler(ctx: Context<FundraiserWithdrawal>, amount: u64) -> Result<()> {
    ctx.accounts.update_config(amount);
    ctx.accounts.transfer_to_fundraiser(amount);
    Ok(())
}




