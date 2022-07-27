use anchor_lang::prelude::*;
use anchor_spl::token::Token;
use solend_utils::SolendDevnet;

use utils::*;
use solend_utils::*;

pub struct YieldContribute {

        #[account(init,
        payer = contributor, 
        space = 50,
        constraint = Clock::get().unwrap().unix_timestamp < fundraiser_config.end_time.try_into().unwrap(),
        )]
        pub contributor_config: Box<Account<'info, Contributor>>,

        #[account(mut)]
        pub fundraiser_config: Box<Account<'info, Fundraiser>>,
        ///Source liquidity token account. [writable]
        ///                $authority can transfer $liquidity_amount.
        #[account(mut)]
        pub fundraiser: Account<'info>,
        //Destination collateral token account. [writable]
        #[account(mut)]
        pub contributor_token_account: Account<'info, TokenAccount>,
        //Reserve account. [writable]
        #[account(mut)]
        pub solend_reserve: AccountInfo<'info>,
        //Reserve liquidity supply SPL Token account. [writable]
        #[account(mut)]
        pub reserve_liqudiity: AccountInfo<'info>,
        ///Reserve collateral SPL Token mint. [writable]
        #[account(mut)]
        pub reserve_mint: Account<'info>,
        ///Lending market account.
        pub lending_market_account: AccountInfo<'info>,
        ///Derived lending market authority.
        pub lending_market_auth: AccountInfo<'info>,
        ///User transfer authority ($authority) [signer].
        #[account(mut)]
        pub contributor: Signer<'info>,
        //Clock sysvar.
        pub clock: Clock<'info>,
        //Token program id.
        pub token_program: AccountInfo<'info>,
        pub token_vault: Account<'info, TokenAccount>,
        pub system_program: Program<'info, System>,
    }

impl<'info> YieldContribute<'info> {
    pub fn create_contributor(nonce: u8) {
        let contribution_pool = &mut ctx.accounts.contribution_pool;
        let contributor = &mut ctx.accounts.contributor;

        contributor.contribution_pool = *ctx.accounts.donation_pool.to_account_info().key;
        contributor.authority = *ctx.accounts.authority.key;
        contributor.balance_deposited = 0;
        contributor.nonce = nonce;

        let pool = &mut ctx.accounts.contribution_pool;
        contributor.user_deposit_count = pool.user_deposit_count.checked_add(1).unwrap();
    }

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

      fn yield_contribute(&mut self) {
        solend_utils::solend_deposit_sol_reserve_liquidity(&amount);
      }

      fn transfer_interest (amount: u64) {

        let interest = ctx.accounts.token_vault.amount - pool.balance_deposited as u128;

      }
}

pub fn handler(ctx: Context<StdContribute>, amount: u64, select_token: u8) {
    ctx.accounts.select(amount, select_token);

    ctx.accounts.yield_contribute(amount);

    ctx.accounts.transfer_interest(interest);

}
