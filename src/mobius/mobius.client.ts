import * as anchor from '@project-serum/anchor';
import { BN, Idl, Program, AnchorProvider, Address } from '@project-serum/anchor';
import { Connection, Keypair, PublicKey, SystemProgram } from '@solana/web3.js';
import {
    Account,
    ASSOCIATED_TOKEN_PROGRAM_ID,
    TOKEN_PROGRAM_ID,
} from '@solana/spl-token';
import { MobiusProtocolPl } from '../../target/types/mobius_protocol_pl';
import { AccountUtils, toBN, isKp, translateAddress } from '../mobius-common';

export class MobiusClient extends AccountUtils {
    // @ts-ignore
    wallet: anchor.Wallet;
    provider!: anchor.Provider;
    mobiusProgram!: anchor.Program<MobiusProtocolPl>;

    constructor(
        conn: Connection,
        // @ts-ignore
        wallet: anchor.Wallet,
        idl?: Idl,
        programId?: PublicKey
    ) {
        super(conn);
        this.wallet = wallet;
        this.setProvider();
        this.setMobiusProtocolPlProgram(idl, programId);
    }

    setProvider() {
        this.provider = new AnchorProvider(
            this.conn,
            this.wallet,
            AnchorProvider.defaultOptions()
        );
        anchor.setProvider(this.provider);
    }

    setMobiusProtocolPlProgram(idl?: Idl, programId?: PublicKey) {
        //instantiating program depends on the environment
        if (idl && programId) {
            //means running in prod
            this.mobiusProgram = new anchor.Program<MobiusProtocolPl>(
                idl as any,
                programId,
                this.provider
            );
        } else {
            //means running inside test suite
            // @ts-ignore
            this.mobiusProgram = anchor.workspace.MobiusProtocolPl as Program<MobiusProtocolPl>;
        }
    }

    // --------------------------------------- fetch deserialized accounts

    async fetchFundraiserAcc(fundraiser: PublicKey) {
        return this.mobiusProgram.account.fundraiser.fetch(fundraiser);
    }

    async fetchContributorAcc(contributor: PublicKey) {
        return this.mobiusProgram.account.contributor.fetch(contributor);
    }

    // --------------------------------------- find PDA addresses

    async findTokenVaultPDA(fundraiserConfig: PublicKey) {
        return await PublicKey.findProgramAddress(
            [Buffer.from(anchor.utils.bytes.utf8.encode("token-vault")), fundraiserConfig.toBytes()],
            this.mobiusProgram.programId
        )
    }

    // async findPlayerFundPDA(player: PublicKey, gameConfig: PublicKey) {
    //     return await PublicKey.findProgramAddress(
    //         [Buffer.from(
    //             anchor.utils.bytes.utf8.encode("player-fund")),
    //         player.toBytes(),
    //         gameConfig.toBytes()
    //         ],
    //         this.mobiusProgram.programId
    //     )
    // }

    // --------------------------------------- find all PDA addresses

    // --------------------------------------- breed ops ixs

    async createFundraiser(
        fundraiserConfig: Keypair,
        fundraiser: Address,
        tokenVault: PublicKey,
        start: number,
        end: number,
        tokenVaultBump: number,
    ) {
        const signers = [fundraiserConfig];
        fundraiser = translateAddress(fundraiser)
        // if (isKp(fundraiser)) signers.push(<Keypair>fundraiser)
        const txSig = await this.mobiusProgram.methods.createFundraiser(
            toBN(start),
            toBN(end),
            tokenVaultBump,
        ).accounts({
            fundraiserConfig: fundraiserConfig.publicKey,
            fundraiser: fundraiser.,
            tokenVault,
            systemProgram: SystemProgram.programId,
            rent: anchor.web3.SYSVAR_RENT_PUBKEY
        }).signers(signers)
            .rpc();
        return { txSig };
    }

    // async joinGame(
    //     gameConfig: PublicKey,
    //     player: PublicKey | Keypair,
    //     playerFund: PublicKey,
    //     playerFundBump: number
    // ) {
    //     const signers = [];
    //     if (isKp(player)) signers.push(<Keypair>player)
    //     const txSig = await this.tradehausProgram.methods.joinGame(
    //         playerFundBump
    //     ).accounts({
    //         gameConfig: gameConfig,
    //         player: isKp(player) ? (<Keypair>player).publicKey : player,
    //         playerFund,
    //         systemProgram: SystemProgram.programId
    //     }).signers(signers)
    //         .rpc();

    //     return { txSig };
    // }

    // async swapItems(
    //     playerFund: PublicKey,
    //     player: PublicKey | Keypair,
    //     gameConfig: PublicKey,
    //     amount: number,
    //     sellCoin: number,
    //     buyCoin: number,
    // ) {
    //     //whenever, this function is called
    //     //check if player is keypair or publickey 
    //     // if it is, push it onto an array/list to store it
    //     //however, will this double count? 
    //     const signers = [];
    //     if (isKp(player)) signers.push(<Keypair>player)
    //     const txSig = await this.tradehausProgram.methods.swapItems(
    //         toBN(amount),
    //         toBN(sellCoin),
    //         toBN(buyCoin),
    //     ).accounts({
    //         playerFund,
    //         player: isKp(player) ? (<Keypair>player).publicKey : player,
    //         gameConfig: gameConfig,
    //         systemProgram: SystemProgram.programId
    //     }).signers(signers)
    //         .rpc();

    //     return { txSig };
    // }
}