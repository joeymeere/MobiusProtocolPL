import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { MobiusProtocolPl } from "../target/types/mobius_protocol_pl";
import { PublicKey, SystemProgram, Transaction, LAMPORTS_PER_SOL } from '@solana/web3.js';
import { TOKEN_PROGRAM_ID, createMint, createAssociatedTokenAccount, mintTo, getAccount } from "@solana/spl-token";
import chai, { assert, AssertionError, expect } from 'chai';
import chaiAspromised from 'chai-as-promised';

import { MorbuisClient } from '../src';

chai.use(chaiAspromised);

describe("morbius", () => {
  //configure the client to use the local cluster 
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const th = new MorbuisClient(provider.connection, provider.wallet as any)

  let fundraiserTokenAccount = null;
  let contributor1TokenAccount = null;
  let contributor2TokenAccount = null;

  const fundraiser = anchor.web3.Keypair.generate();
  const contributor1 = anchor.web3.Keypair.generate();
  const contributor2 = anchor.web3.Keypair.generate();

  const fundraiserConfig = anchor.web3.Keypair.generate();

  it("Initialize token accounts", async () => {
    //airdrop to fundraiser 
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(fundraiser.publicKey, 1000000000),
      "confirmed"
    );

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
        );
        return tx;
      })(),
      [fundraiser]
    );
    // create host & players reward token accounts
    fundraiserTokenAccount = await createAssociatedTokenAccount(
      provider.connection,
      fundraiser,
      null,
      fundraiser.publicKey
    );

    contributor1TokenAccount = await createAssociatedTokenAccount(
      provider.connection,
      contributor1,
      null,
      contributor1.publicKey
    );

    contributor2TokenAccount = await createAssociatedTokenAccount(
      provider.connection,
      contributor2,
      null,
      contributor2.publicKey
    );
  });

  it('creates fundraiser', async () => {
    const [reward_escrow_pda, reward_escrow_bump] = await th.findRewardEscrowPDA(gameConfig.publicKey);

    const _START_TIME = Math.ceil(Date.now() / 1000 + 5)

    await th.createGame(
      gameConfig,
      host,
      hostTokenAccountReward,
      mintReward,
      reward_escrow_pda,
      1,
      _START_TIME,
      2000000000,
      100000,
      3,
      3,
      30,
      reward_escrow_bump
    );

    const gameAcc = await th.fetchGameAcc(gameConfig.publicKey);
    let _rewardEscrow = await getAccount(
      provider.connection,
      reward_escrow_pda
    );

    let _hostTokenAccountReward = await getAccount(
      provider.connection,
      hostTokenAccountReward
    );

    assert.equal(gameAcc.host.toBase58(), host.publicKey.toBase58())
    assert.equal(gameAcc.hostRewardAccount.toBase58(), hostTokenAccountReward.toBase58())
    assert.equal(gameAcc.rewardMint.toBase58(), mintReward.toBase58())
    assert.equal(gameAcc.rewardEscrow.toBase58(), reward_escrow_pda.toBase58())

    assert.ok(gameAcc.rewardAmount.toNumber() == 30)
    assert.ok(gameAcc.joinTime.toNumber() == 1)
    assert.ok(gameAcc.startTime.toNumber() == _START_TIME)
    assert.ok(gameAcc.endTime.toNumber() == 2000000000)
    assert.ok(gameAcc.startUsd.toNumber() == 100000)
    assert.ok(gameAcc.currentCap.toNumber() == 0)
    assert.ok(gameAcc.maxCap.toNumber() == 3)
    assert.ok(gameAcc.winners == 3)
    assert.ok(gameAcc.rewardEscrowBump == reward_escrow_bump)

    const [_vault_authority_pda, _vault_authority_bump] = await PublicKey.findProgramAddress(
      [
        Buffer.from(anchor.utils.bytes.utf8.encode("authority-seed")),
        gameConfig.publicKey.toBytes()
      ],
      th.tradehausProgram.programId
    );

    assert.ok(Number(_hostTokenAccountReward.amount) == 0);
    assert.ok(_rewardEscrow.owner.equals(_vault_authority_pda));
    assert.ok(Number(_rewardEscrow.amount) == 30);


  });





});
