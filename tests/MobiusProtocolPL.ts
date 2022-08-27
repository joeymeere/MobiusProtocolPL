import * as anchor from "@project-serum/anchor";
import { Program, BN, Address } from "@project-serum/anchor";
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

  //create token accounts / mints 
  let solMint = null;
  let solTokenVault: PublicKey = null;
  let solTokenVaultBump = null;
  let fundraiserSolTokenAccount: PublicKey = null;
  let contributor1TokenAccount = null;
  let contributor2TokenAccount = null;

  //generate fundraiser and contributors keypairs  
  const fundraiser = anchor.web3.Keypair.generate();
  const contributor1 = anchor.web3.Keypair.generate();
  const contributor2 = anchor.web3.Keypair.generate();
  const fundraiserConfig = anchor.web3.Keypair.generate();
  const contributorConfig = anchor.web3.Keypair.generate();

  it("Initialize token accounts", async () => {

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
          SystemProgram.transfer({
            fromPubkey: fundraiser.publicKey,
            toPubkey: contributorConfig.publicKey,
            lamports: 100000000,
          }),
        );
        return tx;
      })(),
      [fundraiser]
    );

    // create mint to simulate sol
    solMint = await createMint(
      provider.connection,
      fundraiser,
      fundraiser.publicKey,
      null,
      0
    );

    // create vault pubkey 
    [solTokenVault, solTokenVaultBump] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from('vault'),
        fundraiserConfig.publicKey.toBytes()
      ],
      th.programId
    );

    // solTokenVault = await createAssociatedTokenAccount(
    //   provider.connection,
    //   fundraiser,
    //   solMint,
    //   fundraiser.publicKey
    // );

    // create fundraiser sol token account
    fundraiserSolTokenAccount = await createAssociatedTokenAccount(
      provider.connection,
      fundraiser,
      solMint,
      fundraiser.publicKey
    );

    //create contributor account 
    contributor1TokenAccount = await createAssociatedTokenAccount(
      provider.connection,
      contributor1,
      solMint,
      contributor1.publicKey
    );

    //create contributor account
    contributor2TokenAccount = await createAssociatedTokenAccount(
      provider.connection,
      contributor2,
      solMint,
      contributor2.publicKey
    );

    //mint to contributor token accounts 
    await mintTo(
      provider.connection,
      fundraiser,
      solMint,
      contributor1TokenAccount,
      fundraiser,
      30
    );

    await mintTo(
      provider.connection,
      fundraiser,
      solMint,
      contributor2TokenAccount,
      fundraiser,
      20
    );
  });

  it('creates fundraiser', async () => {

    console.log(fundraiserConfig.publicKey.toBase58());
    console.log(contributorConfig.publicKey.toBase58());
    console.log(fundraiser.publicKey.toBase58());


    // step 1 : pass in accounts created at the start 
    await th.methods
      .createFundraiser(new BN(20))
      .accounts({
        fundraiserConfig: fundraiserConfig.publicKey,
        fundraiser: fundraiser.publicKey,
        fundraiserSolTokenAccount: fundraiserSolTokenAccount,
        tokenVault: solTokenVault,
        solMint: solMint,
        systemProgram: SystemProgram.programId,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        tokenProgram: TOKEN_PROGRAM_ID
      })
      .signers([fundraiserConfig, fundraiser])
      .rpc()
      .catch(console.error);

    // step 2 : fetch the accounts 
    const fundraiserAcc = await th.account.fundraiser.fetch(fundraiserConfig.publicKey);
    // const vault = await getAccount(provider.connection, solTokenVault);

    // step 3 : check that the account state is as expected after passing thru written program instruction
    assert.equal(fundraiserAcc.fundraiser.toBase58(), fundraiser.publicKey.toBase58())
    assert.ok(fundraiserAcc.goal.toNumber() == 20)
    assert.equal(fundraiserAcc.tokenVault.toBase58(), solTokenVault.toBase58())
    assert.equal(fundraiserAcc.solMint.toBase58(), solMint.toBase58())
    assert.equal(fundraiserAcc.fundraiserSolTokenAccount.toBase58(), fundraiserSolTokenAccount.toBase58())

  });

  it('join campaign', async () => {

    //step 1 : pass in accounts required for function 
    await th.methods
      .joinFundraiser()
      .accounts({
        contributorConfig: contributorConfig.publicKey,
        fundraiserConfig: fundraiserConfig.publicKey,
        contributor: contributor1.publicKey,
        systemProgram: SystemProgram.programId,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      })
      .signers([contributor1, contributorConfig])
      .rpc()
      .catch(console.error);

    //step 2 : fetch the accounts 
    const contributorAcc = await th.account.contributor.fetch(contributorConfig.publicKey);

    //step 3: check that the account state is as expected after passing thru written program instruction
    assert.equal(contributorAcc.fundraiserConfig.toBase58(), fundraiserConfig.publicKey.toBase58())
    assert.equal(contributorAcc.contributor.toBase58(), contributor1.publicKey.toBase58())

  });

  it('does fundraiser contribution', async () => {

    //step 1 : pass in accounts required for function   
    await th.methods
      .stdContribute(new BN(10))
      .accounts({
        contributorConfig: contributorConfig.publicKey,
        fundraiserConfig: fundraiserConfig.publicKey,
        contributorTokenAccount: contributor1TokenAccount,
        tokenVault: solTokenVault,
        solMint: solMint,
        contributor: contributor1.publicKey,
        systemProgram: SystemProgram.programId,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        tokenProgram: TOKEN_PROGRAM_ID
      })
      .signers([contributor1])
      .rpc()
      .catch(console.error);

    //step 2 : fetch the accounts 
    const _solTokenVault = await getAccount(provider.connection, solTokenVault);
    const fundraiserAcc = await th.account.fundraiser.fetch(fundraiserConfig.publicKey);
    const contributorAcc = await th.account.contributor.fetch(contributorConfig.publicKey);

    //step 3: check that the account state is as expected after passing thru written program instruction
    assert.ok(Number(_solTokenVault.amount) == 10);
    assert.ok(Number(fundraiserAcc.solQty) == 10);
    assert.ok(Number(contributorAcc.solContributions) == 10);

  });

  it('does fundraiser withdrawal', async () => {

    //step 1 : pass in accounts 
    await th.methods
      .fundraiserWithdrawal(new BN(10))
      .accounts({
        fundraiserConfig: fundraiserConfig.publicKey,
        tokenVault: solTokenVault,
        solMint: solMint,
        fundraiserSolTokenAccount: fundraiserSolTokenAccount,
        systemProgram: SystemProgram.programId,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([fundraiser])
      .rpc()
      .catch(console.error);

    //step 2 : fetch accounts 
    const fundraiser_token_account = await getAccount(provider.connection, fundraiserSolTokenAccount);
    const fundraiserAcc = await th.account.fundraiser.fetch(fundraiserConfig.publicKey);
    const _solTokenVault = await getAccount(provider.connection, solTokenVault);

    //step 3: check that the account state is as expected after passing thru written program instruction 
    assert.ok(Number(_solTokenVault.amount) == 0);
    assert.ok(Number(fundraiserAcc.solQty) == 0);
    assert.ok(Number(fundraiser_token_account.amount) == 10);

  })




});
