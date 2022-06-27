use anchor_lang::prelude::*;

use instructions::*;

pub mod instructions;
pub mod state;

declare_id!("3BBHkxoqwrspCjJ2tqaCAgeUPTBNcKFCWSy5DSQJEhz4");

#[program]
pub mod mobius_protocol_pl {
    use super::*;

    pub fn create_fundraiser(
        ctx: Context<CreateCampaign>, 
        start: u64, 
        end: u64, 
        token_vault_bump: u8,
    ) -> Result<()> {
        instructions::create_fundraiser::handler(
            ctx, 
            start, 
            end, 
            token_vault_bump,
        )
    }

    pub fn std_contribute(
        ctx: Context<StdContribute>, 
        amount: u64,
        select_token: u8 
    ) -> Result<()> {
        instructions::std_contribute::handler(
            ctx,
            amount,
            select_token,  
        )
    }
}