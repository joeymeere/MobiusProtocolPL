use anchor_lang::prelude::*;

use instructions::*;

pub mod instructions;
pub mod state;

declare_id!("5izPbb651w3ZUTgNZnUpmF2bRdzmePAwz4xcnK4NNbEx");

#[program]
pub mod mobius_protocol_pl {
    use super::*;

    pub fn create_fundraiser(
        ctx: Context<CreateCampaign>, 
        start: u64, 
        end: u64, 
        sol_token_vault_bump: u8,
        // usdc_token_vault_bump: u8,
    ) -> Result<()> {
        instructions::create_fundraiser::handler(
            ctx, 
            start, 
            end, 
            sol_token_vault_bump,
            // usdc_token_vault_bump,
        )
    }

    // pub fn std_contribute(
    //     ctx: Context<StdContribute>, 
    //     amount: u128, 
    // ) -> Result<()> {
    //     instructions::std_contribute::handler(
    //         ctx,
    //         amount,  
    //     );
    //     Ok(())
    // }

}