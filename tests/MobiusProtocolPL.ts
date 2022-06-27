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
  let contributor3TokenAccount = null;

  const fundraiser = anchor.web3.Keypair.generate();
  const contributor1 = anchor.web3.Keypair.generate();
  const contributor2 = anchor.web3.Keypair.generate();
  const contributor3 = anchor.web3.Keypair.generate();

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
          SystemProgram.transfer({
            fromPubkey: fundraiser.publicKey,
            toPubkey: contributor3.publicKey,
            lamports: 100000000,
          }),
        );
        return tx;
      })(),
      [fundraiser]
    );
    // create host & players reward token accounts
    hostTokenAccountReward = await createAssociatedTokenAccount(
      provider.connection,
      host,
      mintReward,
      host.publicKey
    );
  });



});
