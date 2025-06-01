// goal is to create a data account on solana blockchain with the help of System Program Contract
import {
  Connection,
  Keypair,
  PublicKey,
  SystemProgram,
  Transaction,
} from "@solana/web3.js";
const connection = new Connection("http://localhost:8899");

async function main() {
  const kp = new Keypair();
  const dataAccount = new Keypair();
  console.log("Public Key:", kp.publicKey.toBase58());
  const tx = await connection.requestAirdrop(kp.publicKey, 1000000000);
  await connection.confirmTransaction(tx);
  console.log("Airdropped 1 SOL to", kp.publicKey.toBase58());

  const ix = SystemProgram.createAccount({
    fromPubkey: kp.publicKey,
    newAccountPubkey: dataAccount.publicKey,
    space: 8,
    lamports: 100000000,
    programId: SystemProgram.programId,
  });
  const tx2 = new Transaction().add(ix);
  tx2.recentBlockhash = (await connection.getLatestBlockhash()).blockhash;
  tx2.feePayer = kp.publicKey;
  const sig = await connection.sendTransaction(tx2, [kp, dataAccount]);
  console.log("Transaction sent:", sig);
  await connection.confirmTransaction(sig);
  console.log("Transaction confirmed");
  console.log("Data Account:", dataAccount.publicKey.toBase58());
  console.log(
    "Data Account Balance:",
    await connection.getBalance(dataAccount.publicKey)
  );
}

main();
