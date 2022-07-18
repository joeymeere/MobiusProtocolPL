use anchor_lang::prelude::*;
use anchor_spl::token::Token;
use solend_utils::SolendDevnet;

use crate::utils::*;
use crate::solend_utils::*;

pub struct YieldContribute {

        #[account(init,
        payer = contributor, 
        space = 50,
        constraint = Clock::get().unwrap().unix_timestamp < fundraiser_config.end_time.try_into().unwrap(),
        )]
        pub contributor_config: Account<'info, Contributor>,

        #[account(mut)]
        pub fundraiser_config: Account<'info, Fundraiser>,

        pub token_vault: Account<'info, TokenAccount>,

        #[account(mut)]
        pub contributor: Signer<'info>,
        pub system_program: Program<'info, System>,
        pub token_program: AccountInfo<'info>,

    ///   0. `[writable]` Source liquidity token account.
    #[account(mut)]
    pub fundraiser: Account<'info>,
    ///                     $authority can transfer $liquidity_amount.
    
    ///   1. `[writable]` Destination collateral token account.
    #[account(mut)]
    pub contributor_token_account: Account<'info, TokenAccount>,

    ///   2. `[writable]` Reserve account.
    #[account(mut)]
    pub solend_reserve: AccountInfo<'info>,

    ///   3. `[writable]` Reserve liquidity supply SPL Token account.
    #[account(mut)]
    pub reserve_liqudiity: AccountInfo<'info>,

    ///   4. `[writable]` Reserve collateral SPL Token mint.
    #[account(mut)]
    pub reserve_mint: Account<'info>,

    ///   5. `[]` Lending market account.
    pub lending_market_account: AccountInfo<'info>,

    ///   6. `[]` Derived lending market authority.
    pub lending_market_auth: AccountInfo<'info>,

    ///   7. `[signer]` User transfer authority ($authority).
    #[account(mut)]
    pub contributor: Signer<'info>,

    ///   8. `[]` Clock sysvar.
    /// 
    ///   9. `[]` Token program id.
    pub token_program: AccountInfo<'info>,

}

impl<'info> YieldContribute<'info> {
    fn select(&mut self, amount: u64, select_token: u8) {
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
            _ => Err(ProgramError::Custom(1)),
          }.unwrap();

      }


      fn yieldcontribute() {

        solend_utils::solend_deposit_sol_reserve_liquidity(&amount);

      }
}

pub fn handler(ctx: Context<StdContribute>, amount: u64, select_token: u8) {
    ctx.accounts.select(amount, select_token);

    ctx.accounts.yieldcontribute(amount);

}
