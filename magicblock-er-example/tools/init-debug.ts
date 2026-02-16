import * as anchor from "@coral-xyz/anchor";
import * as fs from "fs";
import * as os from "os";

async function main() {
  // Construct provider explicitly to avoid requiring ANCHOR_PROVIDER_URL env var
  // Build a wallet from the local Solana CLI keypair file (default path)
  const keypairPath = process.env.ANCHOR_WALLET || `${os.homedir()}/.config/solana/id.json`;
  const keypairData = JSON.parse(fs.readFileSync(keypairPath, "utf8"));
  const kp = anchor.web3.Keypair.fromSecretKey(Uint8Array.from(keypairData));
  const wallet = new anchor.Wallet(kp);

  const provider = new anchor.AnchorProvider(
    new anchor.web3.Connection(process.env.ANCHOR_PROVIDER_URL || "https://api.devnet.solana.com"),
    wallet,
    {},
  );
  anchor.setProvider(provider);
  // Use workspace program object directly (avoid importing generated TS types here)
  const program = anchor.workspace.erStateAccount as any;

  const userAccount = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("user"), wallet.publicKey.toBuffer()],
    program.programId,
  )[0];

  console.log("PDA:", userAccount.toBase58());
  try {
    const sig = await program.methods
      .initialize()
      .accounts({
        user: wallet.publicKey,
        userAccount,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();
    console.log("initialize tx sig:", sig);
  } catch (err: any) {
    console.error("RPC error:", err.toString());
    if (err.logs) {
      console.error("Anchor logs:");
      for (const l of err.logs) console.error(l);
    }
    process.exit(1);
  }
}

main();
