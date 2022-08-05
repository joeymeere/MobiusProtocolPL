use std::str::FromStr;

use crate::utils::*;
use anchor_lang::prelude::*;
use spl_token_lending::instruction::{deposit_reserve_liquidity, redeem_reserve_collateral};

pub fn devnet_solend_deposit_sol_reserve_liquidity(
    liquidity_amount: u64,
    source_liquidity_pubkey: Pubkey,
    destination_collateral_pubkey: Pubkey,
    user_transfer_authority_pubkey: Pubkey,
    signers_seeds: &[&[&[u8]]],
) -> ProgramResult {
    let program_id = Pubkey::from_str(DEVNET_SOLEND_PROGRAM).unwrap();
    let reserve_pubkey = Pubkey::from_str(DEVNET_SOLEND_SOL_RESERVE).unwrap();
    let reserve_liquidity_supply_pubkey =
        Pubkey::from_str(DEVNET_SOLEND_CSOL_LIQUIDITY_SUPPLY).unwrap();
    let reserve_collateral_mint_pubkey =
        Pubkey::from_str(DEVNET_SOLEND_CSOL_COLLATERAL_MINT).unwrap();
    let lending_market_pubkey = Pubkey::from_str(DEVNET_SOLEND_LENDING_MARKET).unwrap();

    let ix = deposit_reserve_liquidity(
        program_id,
        liquidity_amount,
        source_liquidity_pubkey,
        destination_collateral_pubkey,
        reserve_pubkey,
        reserve_liquidity_supply_pubkey,
        reserve_collateral_mint_pubkey,
        lending_market_pubkey,
        user_transfer_authority_pubkey,
    );

    //anchor_lang::solana_program::program::invoke_signed(&ix, &[], signers_seeds)?;

    Ok(())
}

pub fn devnet_solend_redeem_reserve_collateral(
    collateral_amount: u64,
    source_collateral_pubkey: Pubkey,
    destination_liqudity_pubkey: Pubkey,
    user_transfer_authority_pubkey: Pubkey,
    signers_seeds: &[&[&[u8]]],
) -> ProgramResult {

    let reserve_collateral_supply_pubkey =
    Pubkey::from_str(DEVNET_SOLEND_CSOL_LIQUIDITY_SUPPLY).unwrap();
    let reserve_liquidity_mint_pubkey =
    Pubkey::from_str(DEVNET_SOLEND_CSOL_COLLATERAL_MINT).unwrap();

    let ix = redeem_reserve_collateral(
        program_id,
        collateral_amount,
        source_collateral_pubkey,
        destination_liquidity_pubkey,
        reserve_pubkey,
        reserve_collateral_supply_pubkey,
        reserve_liquidity_mint_pubkey,
        lending_market_pubkey,
        user_transfer_authority_pubkey,
);

}

#[derive(Clone)]
pub struct SolendDevnet;

impl anchor_lang::Id for SolendDevnet {
    fn id() -> Pubkey {
        Pubkey::from_str(DEVNET_SOLEND_PROGRAM).unwrap()
    }
}