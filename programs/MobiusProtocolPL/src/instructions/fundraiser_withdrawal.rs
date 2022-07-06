use anchor_lang::prelude::*;

use crate::state::*;
use anchor_spl::token::{self, TokenAccount};
use spl_token::instruction::AuthorityType;
use std::mem::size_of;

#[derive(Accounts)]
pub struct Withdrawal<'info> {
    // this is a config so don't put seeds here
    #[account(
        init, 
        payer = withdrawer,
        space = size_of::<fundraiser>() + 8
    )]
    pub Fundraiser: Account<'info, Fundraiser>,

    // put seeds for the escrow account
    #[account(
        init,
        payer = user_sending,
        seeds=[b"withdraw".as_ref(), withdrawer.key().as_ref(), mint_of_token_being_sent.key().as_ref()],
        bump,
        token::mint=mint_of_token_being_sent,
        token::authority=Fundraiser,
    )]
<<<<<<< Updated upstream
    escrow_wallet_state: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = Fundraiser.fundraiser = wallet_to_withdraw_to.key(), // ensures that no shady stuff happens, security check (only one who can withdraw is fundraiser creator)
=======
    token_vault: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = Fundraiser.fundraiser = wallet_to_withdraw_to.key(), // ensures that no shady stuff happens, security check (only one who can withdraw is fundraiser creatorj)
>>>>>>> Stashed changes
        constraint=wallet_to_withdraw_from.mint == mint_of_token_being_sent.key() // some minting stuff i saw
    )]

    //wallet of the withdrawer
    wallet_to_withdraw_to: Account<'info, TokenAccount>,
    mint_of_token_being_sent: Account<'info, Mint>,

    #[account(mut)]
    pub withdrawer_token_account: Account<'info, TokenAccount>,
<<<<<<< Updated upstream
    pub token_vault: Account<'info, TokenAccount>,
=======
>>>>>>> Stashed changes

    #[account(mut)]
    pub withdrawer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: AccountInfo<'info>,
}

impl<'info> Withdrawal<'info> {

<<<<<<< Updated upstream
    pub fn transfer_to_escrow(&mut self, select_token: u8) {

        let mint_of_token_being_sent_pk = ctx.accounts.mint_of_token_being_sent.key().clone();

        let inner = vec![
            b"withdraw".as_ref(),
            ctx.accounts.withdrawer.key.as_ref(),
            mint_of_token_being_sent_pk.as_ref(),
        ];

        let outer = vec![inner.as_slice()];

        // this initializes the seeds since they are a parameter

        let transfer_instruction = Transfer{
            from: ctx.accounts.Fundraiser.to_account_info(),
            to: ctx.accounts.escrow_wallet_state.to_account_info(),
            authority: ctx.accounts.withdrawer.to_account_info(),
        };

        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            transfer_instruction,
            outer.as_slice(),
        );

        // once we have this, we will do the transfer to the escrow only based on the selected token, so we will set up a match statement below with a default wall cut
        // first make sure the fundraiser is going to be empty
        match select_token {
            1 => {
                self.Fundraiser.sol_qty = 0;
                Ok(self.Fundraiser.sol_qty);
                anchor_spl::token::transfer(cpi_ctx, self.Fundraiser.sol_qty)?;
                },

            2 => {
                self.Fundraiser.usdc_qty = 0;
                Ok(self.Fundraiser.usdc_qty);
                anchor_spl::token::transfer(cpi_ctx, self.Fundraiser.usdc_qty)?;
                
                },

            3 => {
                self.Fundraiser.usdt_qty = 0; 
                Ok(self.Fundraiser.usdt_qty);
                anchor_spl::token::transfer(cpi_ctx, self.Fundraiser.usdt_qty)?;
                 
                },

        }.unwrap();


    }

=======
>>>>>>> Stashed changes
    pub fn escrow_to_withdrawer(&mut self, select_token: u8) {

        let mint_of_token_being_sent_pk = ctx.accounts.mint_of_token_being_sent.key().clone();

        let inner = vec![
            b"withdraw".as_ref(),
            ctx.accounts.withdrawer.key.as_ref(),
            mint_of_token_being_sent_pk.as_ref(),
        ];

        let outer = vec![inner.as_slice()];

        // this initializes the seeds since they are a parameter

        let transfer_instruction = Transfer{
<<<<<<< Updated upstream
            from: ctx.accounts.escrow_wallet_state.to_account_info(),
=======
            from: ctx.accounts.token_vault.to_account_info(),
>>>>>>> Stashed changes
            to: ctx.accounts.wallet_to_withdraw_to.to_account_info(),
            authority: ctx.accounts.withdrawer.to_account_info(),
        };

        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            transfer_instruction,
            outer.as_slice(),
        );

        // once we have this, we will do the transfer to the escrow only based on the selected token, so we will set up a match statement below with a default wall cut
        // first make sure the fundraiser is going to be empty
        match select_token {
            1 => {
<<<<<<< Updated upstream
=======
                self.Fundraiser.sol_qty = 0;
                Ok(self.Fundraiser.sol_qty);
>>>>>>> Stashed changes
                self.Withdrawer.sol_qty += self.Fundraiser.sol_qty;
                Ok(self.Withdrawer.sol_qty);
                anchor_spl::token::transfer(cpi_ctx, self.Fundraiser.sol_qty)?;
                },

            2 => {
<<<<<<< Updated upstream
=======
                self.Fundraiser.usdc_qty = 0;
                Ok(self.Fundraiser.usdc_qty);
>>>>>>> Stashed changes
                self.Withdrawer.sol_qty += self.Fundraiser.usdc_qty;
                Ok(self.Withdrawer.usdc_qty);
                anchor_spl::token::transfer(cpi_ctx, self.Fundraiser.usdc_qty)?;
                },

            3 => {
<<<<<<< Updated upstream
=======
                self.Fundraiser.usdt_qty = 0; 
                Ok(self.Fundraiser.usdt_qty);
>>>>>>> Stashed changes
                self.Withdrawer.sol_qty += self.Fundraiser.usdt_qty;
                Ok(self.Withdrawer.usdt_qty);
                anchor_spl::token::transfer(cpi_ctx, self.Fundraiser.usdt_qty)?; 
                },

        }.unwrap();


    }

}

pub fn handler(ctx: Context<Withdrawal>, select_token: u8) {
<<<<<<< Updated upstream

    ctx.accounts.transfer_to_escrow(select_token);
    ctx.accounts.escrow_to_withdrawer(select_token);

=======
    ctx.accounts.escrow_to_withdrawer(select_token);
>>>>>>> Stashed changes
}