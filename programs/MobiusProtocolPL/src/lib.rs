use anchor_lang::prelude::*;

use instructions::*;

pub mod instructions;
pub mod state;

declare_id!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");

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
}