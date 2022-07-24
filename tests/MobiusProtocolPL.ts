import * as anchor from "@project-serum/anchor";
import { Program, BN } from "@project-serum/anchor";
import { MobiusProtocolPl } from "../target/types/mobius_protocol_pl";
import { Connection, PublicKey, SystemProgram, Transaction, LAMPORTS_PER_SOL } from '@solana/web3.js';
import { TOKEN_PROGRAM_ID, createMint, createAssociatedTokenAccount, mintTo, getAccount } from "@solana/spl-token";
import chai, { assert, AssertionError, expect } from 'chai';
import chaiAspromised from 'chai-as-promised';

// import { MobiusClient } from '../src';

chai.use(chaiAspromised);

describe("mobius", () => {
  //configure the client to use the local cluster 
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const th = anchor.workspace.MobiusProtocolPl as Program<MobiusProtocolPl>;

  // const th = new MobiusClient(provider.connection, provider.wallet as any)

  let solMint = null;
  let usdcMint = null;

  let fundraiserSolTokenAccount = null;
  let fundraiserUsdcTokenAccount = null;
  let contributor1TokenAccount = null;
  let contributor2TokenAccount = null;

  let solTokenVault = null;
  let sol_token_vault_pda: PublicKey = null; // escrow account stores reward tokens
  let sol_token_vault_bump: number = null;
  // let sol_token_vault_authority_pda = null;

  let usdcTokenVault = null;
  let usdc_token_vault_pda: PublicKey = null; // escrow account stores reward tokens
  let usdc_token_vault_bump: number = null;
  // let usdc_token_vault_authority_pda = null;

  let vault_authority_pda = null;


  const fundraiser = anchor.web3.Keypair.generate();
  const contributor1 = anchor.web3.Keypair.generate();
  const contributor2 = anchor.web3.Keypair.generate();

  const fundraiserConfig = anchor.web3.Keypair.generate();

  it("Initialize token accounts", async () => {
    //airdrop to fundraiser

    //deprecated 
    // await provider.connection.confirmTransaction(
    //   await provider.connection.requestAirdrop(fundraiser.publicKey, 1000000000),
    //   "processed"
    // );

    const connection = new Connection("http://127.0.0.1:8899", "confirmed");
    const airdropSignature = await connection.requestAirdrop(
      fundraiser.publicKey,
      1000000000
    );

    const latestBlockHash = await connection.getLatestBlockhash();

    await connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: airdropSignature,
    });

    //fund contributors account
    await provider.sendAndConfirm(
      (() => {
        const tx = new Transaction();
        tx.add(
          SystemProgram.transfer({
            fromPubkey: fundraiser.publicKey,
            toPubkey: contributor1.publicKey,
            lamports: 100000000,
          }),
          SystemProgram.transfer({
            fromPubkey: fundraiser.publicKey,
            toPubkey: contributor2.publicKey,
            lamports: 100000000,
          }),
          SystemProgram.transfer({
            fromPubkey: fundraiser.publicKey,
            toPubkey: fundraiserConfig.publicKey,
            lamports: 100000000,
          }),
        );
        return tx;
      })(),
      [fundraiser]
    );

    // create mint of reward token
    solMint = await createMint(
      provider.connection,
      fundraiser,
      fundraiser.publicKey,
      null,
      0
    );

    usdcMint = await createMint(
      provider.connection,
      fundraiser,
      fundraiser.publicKey,
      null,
      0
    );

    // create fundraiser sol token account
    fundraiserSolTokenAccount = await createAssociatedTokenAccount(
      provider.connection,
      fundraiser,
      solMint,
      fundraiser.publicKey
    );

    // create fundraiser usdc token account
    fundraiserUsdcTokenAccount = await createAssociatedTokenAccount(
      provider.connection,
      fundraiser,
      usdcMint,
      fundraiser.publicKey
    );

    // create fundraiser usdc token account
    solTokenVault = await createAssociatedTokenAccount(
      provider.connection,
      fundraiser,
      solMint,
      fundraiserConfig.publicKey
    );

    // create fundraiser usdc token account
    usdcTokenVault = await createAssociatedTokenAccount(
      provider.connection,
      fundraiser,
      usdcMint,
      fundraiserConfig.publicKey
    );

    contributor1TokenAccount = await createAssociatedTokenAccount(
      provider.connection,
      contributor1,
      solMint,
      contributor1.publicKey
    );

    contributor2TokenAccount = await createAssociatedTokenAccount(
      provider.connection,
      contributor2,
      usdcMint,
      contributor2.publicKey
    );
  });

  it('creates fundraiser', async () => {
    const [_sol_token_vault_pda, _sol_token_vault_bump] = await PublicKey.findProgramAddress(
      [Buffer.from(anchor.utils.bytes.utf8.encode("sol-token-vault")), fundraiserConfig.publicKey.toBytes()],
      th.programId
    );

    const [_usdc_token_vault_pda, _usdc_token_vault_bump] = await PublicKey.findProgramAddress(
      [Buffer.from(anchor.utils.bytes.utf8.encode("usdc-token-vault")), fundraiserConfig.publicKey.toBytes()],
      th.programId
    );

    const _START_TIME = Math.ceil(Date.now() / 1000 + 5);

    sol_token_vault_pda = _sol_token_vault_pda;
    sol_token_vault_bump = _sol_token_vault_bump;

    usdc_token_vault_pda = _usdc_token_vault_pda;
    usdc_token_vault_bump = _usdc_token_vault_bump;

    console.log(fundraiser.publicKey.toBase58());
    console.log(fundraiserConfig.publicKey.toBase58());
    console.log(sol_token_vault_pda.toBase58());
    console.log(usdc_token_vault_pda.toBase58());
    console.log(solMint.toBase58());
    console.log(usdcMint.toBase58());
    console.log(fundraiserSolTokenAccount.toBase58());
    console.log(fundraiserUsdcTokenAccount.toBase58());

    await th.methods
      .createFundraiser(new BN(_START_TIME), new BN(2000000000), sol_token_vault_bump, usdc_token_vault_bump)
      .accounts({
        fundraiserConfig: fundraiserConfig.publicKey,
        fundraiser: fundraiser.publicKey,
        solTokenVault: sol_token_vault_pda,
        usdcTokenVault: usdc_token_vault_pda,
        solMint: solMint,
        usdcMint: usdcMint,
        fundraiserSolTokenAccount: fundraiserSolTokenAccount,
        fundraiserUsdcTokenAccount: fundraiserUsdcTokenAccount,
        systemProgram: SystemProgram.programId,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        tokenProgram: TOKEN_PROGRAM_ID
      })
      .signers([fundraiserConfig, fundraiser])
      .rpc()
      .catch(console.error);

    // this test is for 'set_fundraiser_config' function
    const fundraiserAcc = await th.account.fundraiser.fetch(fundraiserConfig.publicKey);

    let _solTokenVault = await getAccount(
      provider.connection,
      sol_token_vault_pda
    );

    let _usdcTokenVault = await getAccount(
      provider.connection,
      usdc_token_vault_pda
    );

    // let _hostTokenAccountReward = await getAccount(
    //   provider.connection,
    //   hostTokenAccount
    // );

    assert.equal(fundraiserAcc.fundraiser.toBase58(), fundraiser.publicKey.toBase58())
    // assert.equal(fundraiserAcc.hostRewardAccount.toBase58(), hostTokenAccountReward.toBase58())
    assert.equal(fundraiserAcc.solTokenVault.toBase58(), sol_token_vault_pda.toBase58())
    assert.equal(fundraiserAcc.usdcTokenVault.toBase58(), usdc_token_vault_pda.toBase58())
    assert.ok(fundraiserAcc.startTime.toNumber() == _START_TIME)
    assert.ok(fundraiserAcc.endTime.toNumber() == 2000000000)
    assert.ok(fundraiserAcc.solTokenVaultBump == sol_token_vault_bump)
    assert.ok(fundraiserAcc.usdcTokenVaultBump == usdc_token_vault_bump)

    // this test is for 'set_authority_token_vault' function
    const [_vault_authority_pda, _vault_authority_bump] = await PublicKey.findProgramAddress(
      [
        Buffer.from(anchor.utils.bytes.utf8.encode("authority-seed")),
        fundraiserConfig.publicKey.toBytes()
      ],
      th.programId
    );

    vault_authority_pda = _vault_authority_pda

    // assert.ok(Number(_hostTokenAccountReward.amount) == 0);
    assert.ok(_solTokenVault.owner.equals(vault_authority_pda));
    assert.ok(_usdcTokenVault.owner.equals(vault_authority_pda));
    // assert.ok(Number(_rewardEscrow.amount) == 30);
  });





});
