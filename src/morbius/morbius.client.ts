import { Connection, Keypair, PublicKey, SystemProgram } from '@solana/web3.js';
import {
    Account,
    ASSOCIATED_TOKEN_PROGRAM_ID,
    TOKEN_PROGRAM_ID,
} from '@solana/spl-token';
import { MobiusProtocolPl } from '../../target/types/mobius_protocol_pl';
import { AccountUtils, toBN, isKp } from '../morbius-common';

export class TradehausClient extends AccountUtils {
    // @ts-ignore
    wallet: anchor.Wallet;
    provider!: anchor.Provider;
    tradehausProgram!: anchor.Program<Tradehaus>;

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
        this.setTradehausProgram(idl, programId);
    }

    setProvider() {
        this.provider = new AnchorProvider(
            this.conn,
            this.wallet,
            AnchorProvider.defaultOptions()
        );
        anchor.setProvider(this.provider);
    }

    setTradehausProgram(idl?: Idl, programId?: PublicKey) {
        //instantiating program depends on the environment
        if (idl && programId) {
            //means running in prod
            this.tradehausProgram = new anchor.Program<Tradehaus>(
                idl as any,
                programId,
                this.provider
            );
        } else {
            //means running inside test suite
            // @ts-ignore
            this.tradehausProgram = anchor.workspace.Tradehaus as Program<Tradehaus>;
        }
    }

    // --------------------------------------- fetch deserialized accounts

    async fetchGameAcc(game: PublicKey) {
        return this.tradehausProgram.account.game.fetch(game);
    }

    async fetchFundAcc(fund: PublicKey) {
        return this.tradehausProgram.account.fund.fetch(fund);
    }

    // --------------------------------------- find PDA addresses

    async findRewardEscrowPDA(gameConfig: PublicKey) {
        return await PublicKey.findProgramAddress(
            [Buffer.from(anchor.utils.bytes.utf8.encode("reward-escrow")), gameConfig.toBytes()],
            this.tradehausProgram.programId
        )
    }

    async findPlayerFundPDA(player: PublicKey, gameConfig: PublicKey) {
        return await PublicKey.findProgramAddress(
            [Buffer.from(
                anchor.utils.bytes.utf8.encode("player-fund")),
            player.toBytes(),
            gameConfig.toBytes()
            ],
            this.tradehausProgram.programId
        )
    }

    // --------------------------------------- find all PDA addresses

    // --------------------------------------- breed ops ixs

    async createGame(
        gameConfig: Keypair,
        host: PublicKey | Keypair,
        hostRewardAccount: PublicKey,
        rewardMint: PublicKey,
        rewardEscrow: PublicKey,
        join: number,
        start: number,
        end: number,
        startUsd: number,
        winners: number,
        maxPlayers: number,
        rewardAmount: number,
        rewardEscrowBump: number
    ) {
        const signers = [gameConfig];
        if (isKp(host)) signers.push(<Keypair>host)
        const txSig = await this.tradehausProgram.methods.createGame(
            toBN(join),
            toBN(start),
            toBN(end),
            toBN(startUsd),
            winners,
            toBN(maxPlayers),
            toBN(rewardAmount),
            rewardEscrowBump
        ).accounts({
            gameConfig: gameConfig.publicKey,
            host: isKp(host) ? (<Keypair>host).publicKey : host,
            hostRewardAccount,
            rewardMint,
            rewardEscrow,
            systemProgram: SystemProgram.programId,
            rent: anchor.web3.SYSVAR_RENT_PUBKEY
        }).signers(signers)
            .rpc();

        return { txSig };
    }

    async joinGame(
        gameConfig: PublicKey,
        player: PublicKey | Keypair,
        playerFund: PublicKey,
        playerFundBump: number
    ) {
        const signers = [];
        if (isKp(player)) signers.push(<Keypair>player)
        const txSig = await this.tradehausProgram.methods.joinGame(
            playerFundBump
        ).accounts({
            gameConfig: gameConfig,
            player: isKp(player) ? (<Keypair>player).publicKey : player,
            playerFund,
            systemProgram: SystemProgram.programId
        }).signers(signers)
            .rpc();

        return { txSig };
    }

    async swapItems(
        playerFund: PublicKey,
        player: PublicKey | Keypair,
        gameConfig: PublicKey,
        amount: number,
        sellCoin: number,
        buyCoin: number,
    ) {
        //whenever, this function is called
        //check if player is keypair or publickey 
        // if it is, push it onto an array/list to store it
        //however, will this double count? 
        const signers = [];
        if (isKp(player)) signers.push(<Keypair>player)
        const txSig = await this.tradehausProgram.methods.swapItems(
            toBN(amount),
            toBN(sellCoin),
            toBN(buyCoin),
        ).accounts({
            playerFund,
            player: isKp(player) ? (<Keypair>player).publicKey : player,
            gameConfig: gameConfig,
            systemProgram: SystemProgram.programId
        }).signers(signers)
            .rpc();

        return { txSig };
    }
}