import { BN } from 'bn.js';
import { Buffer } from 'buffer';
import idl from './idl.json';
import './App.css';
//import kp from './keypair.json'
import React, { useEffect, useState } from 'react';
import { Connection, PublicKey, clusterApiUrl } from '@solana/web3.js';
import { Program, Provider, web3 } from '@project-serum/anchor';
import Header from './components/Header';
import Card from './components/Card';
import Form from './components/Form';
import { getAllCampaigns, getWithdrawalData } from "./solana";
import { PhantomWalletAdapter } from '@solana/wallet-adapter-wallets';
import { useWallet, WalletProvider, ConnectionProvider } from '@solana/wallet-adapter-react';
import { WalletModalProvider, WalletMultiButton } from '@solana/wallet-adapter-react-ui';
require('@solana/wallet-adapter-react-ui/styles.css');


const wallets = [
  /* view list of available wallets at https://github.com/solana-labs/wallet-adapter#wallets */
  new PhantomWalletAdapter()
]

// SystemProgram is a reference to the Solana runtime!
const { SystemProgram, Keypair } = web3;

// Get our program's id from the IDL file.
const programID = new PublicKey(idl.metadata.address);

// Set our network to devnet.
const network = clusterApiUrl('devnet');

// Controls how we want to acknowledge when a transaction is "done".
const opts = {
  preflightCommitment: "confirmed"
}

const donator = Keypair.generate();

export async function createCampaign(amount) {


  window.Buffer = Buffer;
  const provider = getProvider();
  const program = new Program(idl, programID, provider);
  const { writingAccount, bump } = await getProgramDerivedCampainWritingAccountAddress();

  // step 1 : pass in accounts created at the start 
  await th.methods
    .createFundraiser(amount)
    .accounts({
      fundraiserConfig: fundraiserConfig.publicKey,
      fundraiser: fundraiser.publicKey,
      fundraiserTokenAccount: fundraiserTokenAccount,
      tokenVault: solTokenVault,
      solMint: solMint,
      systemProgram: SystemProgram.programId,
      rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      tokenProgram: TOKEN_PROGRAM_ID
    })
    .signers([fundraiserConfig, fundraiser])
    .rpc()
    .catch(console.error);
}

export async function joinCampaign() {

  window.Buffer = Buffer;
  const provider = getProvider();
  const program = new Program(idl, programID, provider);
  const { donatorProgramAccount, bump } = await getProgramDerivedCampainDonatorProgramAccountAddress();

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
}


export async function donateToCampaign(campaignPubKey, amount) {

  window.Buffer = Buffer;
  const provider = getProvider();
  const program = new Program(idl, programID, provider);
  const { donatorProgramAccount, bump } = await getProgramDerivedCampainDonatorProgramAccountAddress();

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

}

alert("Your Donation transaction signature", donateTx);
let account = await program.account.donatorProgramAccount.fetch(donatorProgramAccount);
console.log("👀 Created a New Donator Program Account : ", account);
alert("Donation Successful");
}


export async function withdraw(campaignPubKey, amount) {

  window.Buffer = Buffer;
  const provider = getProvider();
  const program = new Program(idl, programID, provider);
  await th.methods
    .fundraiserWithdrawal(new BN(10))
    .accounts({
      fundraiserConfig: fundraiserConfig.publicKey,
      tokenVault: solTokenVault,
      solMint: solMint,
      fundraiserTokenAccount: fundraiserTokenAccount,
      systemProgram: SystemProgram.programId,
      rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      tokenProgram: TOKEN_PROGRAM_ID,
    })
    .signers([])
    .rpc()
    .catch(console.error);
}

let withdrawalData = await getWithdrawalData();
console.log(withdrawalData);
console.log("New Withdrawal Successful ! Initiated by : ", provider.wallet.publicKey);

}


async function getProgramDerivedCampainWritingAccountAddress() {
  const provider = getProvider();
  const [writingAccount, bump] = await PublicKey.findProgramAddress(
    [Buffer.from('please_____'), provider.wallet.publicKey.toBuffer()],
    programID
  );

  console.log(`Got ProgramDerivedWritingAccountAddress: bump: ${bump}, pubkey: ${writingAccount.toBase58()}`);
  return { writingAccount, bump };

};

async function getProgramDerivedCampainDonatorProgramAccountAddress() {

  const [donatorProgramAccount, bump] = await PublicKey.findProgramAddress(
    [Buffer.from('donate____'), donator.publicKey.toBuffer()],
    programID
  );
  console.log(`Got ProgramDerivedDonatorProgramAccountAddress: bump: ${bump}, pubkey: ${donatorProgramAccount.toBase58()}`);
  return { donatorProgramAccount, bump };

};


async function getProvider() {
  const connection = new Connection(network, opts.preflightCommitment);

  const provider = new Provider(
    connection, window.solana, opts.preflightCommitment,
  );
  return provider;
}



function App() {
  return (
    <div className="App">
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
        <p>
          Edit <code>src/App.js</code> and save to reload.
        </p>
        <a
          className="App-link"
          href="https://reactjs.org"
          target="_blank"
          rel="noopener noreferrer"
        >
          Learn React
        </a>
      </header>
    </div>
  );
}

export default App;
