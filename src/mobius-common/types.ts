import { BN } from '@project-serum/anchor';
import { Keypair, PublicKey } from '@solana/web3.js';

export function isKp(toCheck: PublicKey | Keypair) {
    return typeof (<Keypair>toCheck).publicKey !== 'undefined';
}

export function toBN(i: any): any {
    if (typeof i == 'number') {
        return new BN(i);
    } else if (i instanceof BN) {
        return i;
    } else if (parseType(i) === 'array') {
        const bnArray = [];

        for (const item in i) {
            bnArray.push(toBN(item));
        }

        return bnArray;
    } else if (parseType(i) === 'object') {
        const bnObj = {};

        for (const field in i) {
            // @ts-ignore
            bnObj[field] = toBN(i[field]);
        }

        return bnObj;
    }

    return i;
}

// Translates an address to a Pubkey.
export function translateAddress(address: Address): PublicKey {
    return address instanceof PublicKey ? address : new PublicKey(address);
}

/**
 * An address to identify an account on chain. Can be a [[PublicKey]],
 * or Base 58 encoded string.
 */
export type Address = PublicKey | string;

function parseType<T>(v: T): string {
    if (v === null || v === undefined) {
        return 'null';
    }
    if (typeof v === 'object') {
        if (v instanceof Array) {
            return 'array';
        }
        if (v instanceof Date) {
            return 'date';
        }
        return 'object';
    }
    return typeof v;
}