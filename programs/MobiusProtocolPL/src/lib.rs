use anchor_lang::prelude::*;

use instructions::*;

pub mod instructions;
pub mod state;


declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod mobius_protocol_pl {
    use super::*;

    pub fn create_contributor(ctx:Context<YieldContribute>, nonce: u8) -> Result<()> {
        instructions::create_contributor::handler(ctx, nonce)
    }

    pub fn deposit(ctx:Context<YieldContribute>, amount: u64) -> Result<()> {
        instructions::deposit::handler(ctx, cpi_ctx, amount)
    }

    pub fn transfer_interest(ctx:Context<YieldContribute>, contribution: u128) -> Result<()> {
        instructions::transfer_interest::handler(ctx, cpi_ctx, contribution)
    }

    pub fn withdraw(ctx:Context<YieldContribute>, contribution: u128) -> Result<()> {
        instructions::yieldwithdraw::handler(ctx, cpi_ctx, principal)
    }
}

#[derive(Accounts)]
pub struct Initialize {}
