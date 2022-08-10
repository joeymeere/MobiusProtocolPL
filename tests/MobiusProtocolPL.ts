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
  let fundraiserSolTokenAccount = null;
  let contributor1TokenAccount = null;
  let contributor2TokenAccount = null;
  let solTokenVault = null;
  let vault_authority_pda: PublicKey = null;

  //const donator amount 


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
      0,
    );

    // create fundraiser sol token account
    fundraiserSolTokenAccount = await createAssociatedTokenAccount(
      provider.connection,
      fundraiser,
      solMint,
      fundraiser.publicKey
    );

    // create solTokenVault
    solTokenVault = await createAssociatedTokenAccount(
      provider.connection,
      fundraiser,
      solMint,
      fundraiserConfig.publicKey
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

    const _START_TIME = Math.ceil(Date.now() / 1000 + 5);

    // testing purposes for checking if any acocunts are missing... 
    // console.log(fundraiser.publicKey.toBase58());

    // step 1 : pass in accounts created at the start 
    await th.methods
      .createFundraiser(new BN(_START_TIME), new BN(2000000000))
      .accounts({
        fundraiserConfig: fundraiserConfig.publicKey,
        fundraiser: fundraiser.publicKey,
        solTokenVault: solTokenVault,
        fundraiserSolTokenAccount: fundraiserSolTokenAccount,
        systemProgram: SystemProgram.programId,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        tokenProgram: TOKEN_PROGRAM_ID
      })
      .signers([fundraiserConfig, fundraiser])
      .rpc()
      .catch(console.error);

    // step 2 : fetch the accounts 
    const fundraiserAcc = await th.account.fundraiser.fetch(fundraiserConfig.publicKey);
    const _solTokenVault = await getAccount(provider.connection, solTokenVault);

    // step 3 : check that the account state is as expected after passing thru written program instruction
    assert.equal(fundraiserAcc.fundraiser.toBase58(), fundraiser.publicKey.toBase58())
    assert.ok(fundraiserAcc.startTime.toNumber() == _START_TIME)
    assert.ok(fundraiserAcc.endTime.toNumber() == 2000000000)

    const [_vault_authority_pda, _vault_authority_bump] = await PublicKey.findProgramAddress(
      [
        Buffer.from(anchor.utils.bytes.utf8.encode("authority-seed")),
        fundraiserConfig.publicKey.toBytes()
      ],
      th.programId
    );

    vault_authority_pda = _vault_authority_pda;

    assert.ok(_solTokenVault.owner.equals(vault_authority_pda));
  });

  it('does standard contribution', async () => {

    //step 1 : pass in accounts required for function 
    await th.methods
      .stdContribute(new BN(10))
      .accounts({
        contributorConfig: contributorConfig.publicKey,
        fundraiserConfig: fundraiserConfig.publicKey,
        contributorTokenAccount: contributor1TokenAccount,
        solTokenVault: solTokenVault,
        contributor: contributor1.publicKey,
        systemProgram: SystemProgram.programId,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        tokenProgram: TOKEN_PROGRAM_ID
      })
      .signers([contributor1, contributorConfig])
      .rpc()
      .catch(console.error);

    //step 2 : fetch the accounts 
    const fundraiserAcc = await th.account.fundraiser.fetch(fundraiserConfig.publicKey);
    const _solTokenVault = await getAccount(provider.connection, solTokenVault);
    const contributorAcc = await th.account.contributor.fetch(contributorConfig.publicKey);

    //step 3: check that the account state is as expected after passing thru written program instruction
    assert.ok(Number(_solTokenVault.amount) == 10);
    assert.ok(fundraiserAcc.solQty.toNumber() == 10);
    assert.ok(contributorAcc.solContributions.toNumber() == 10);

  });



});
