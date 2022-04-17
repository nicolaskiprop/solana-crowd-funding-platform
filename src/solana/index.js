import Wallet from "@project-serum/sol-wallet-adapter";
import  { Connection, SystemProgram, Transaction, PublicKey, TransactionInstruction } from "solana/web3.js";
import { deserialize, serialize }  from "borsh";

const cluster = "https://api.devnet.solana.com";
const connection = new Connection(cluster, "confirmed");
const wallet = new Wallet("https://www.sollet.io", cluster);
const programId = new PublicKey("286rapsUbvDe1ZgBeNhp37YHvEPwWPTr4Bkce4oMpUKT");





export async function setPayerAndBlockhashTransaction(instructions) {
    const transaction = new Transaction();
    instructions.forEach(element => {
        transaction.add(element);
    });
    transaction.feePayer = wallet.PublicKey;
    let hash = await connection.getRecentBlockhash();
    transaction.recentBlockhash = hash.blockhash;
    return transaction;
}

export async function signAndSendTransaction(transaction) {
    try {
        console.log("start signAndSendTransaction");
        let signedTrans = await wallet.signTransaction(transaction);
        console.log(("signed transaction"));
        let signature = await connection.sendRawTransaction(
            signedTrans.serialize()
        );
        console.log("end signAndSendTransaction");
        return signature;
    }   catch (err) {
        console.log("signAndSendTransaction error", err);
        throw err;
    }
}

//invoke create_campaign instruction

class CampaignDetails {
    constructor(properties) {
        Object.keys(properties).forEach((key) => {
            this[key] = properties[key];
        });
    }
    static schema = new Map([[CampaignDetails,{
        kind:'struct',
        fields: [
            ['admin',[32]],
            ['name', 'string'],
            ['description', 'string'],
            ['image_link', 'string'],
            ['amount_donated', 'u64']
        ]
    }]]);
}