// tests/define-token.ts
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Habitdao } from "../target/types/habitdao";
import {
TOKEN_2022_PROGRAM_ID,
getAssociatedTokenAddress,
} from "@solana/spl-token";
import { PublicKey } from "@solana/web3.js";

describe("define-token", () => {
const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);

const program = anchor.workspace.Habitdao as Program<Habitdao>;
const admin = provider.wallet.publicKey;

// Use your created mint
const mint = new PublicKey(process.env.HABIT_TOKEN_MINT!);

it("Initialize DAO with Token-2022", async () => {
const [daoConfig] = PublicKey.findProgramAddressSync(
[Buffer.from("dao_config")],
program.programId
);

const [vaultAuthority] = PublicKey.findProgramAddressSync(
  [Buffer.from("vault_authority"), daoConfig.toBuffer()],
  program.programId
);

const vaultTokenAccount = await getAssociatedTokenAddress(
  mint,
  vaultAuthority,
  true,
  TOKEN_2022_PROGRAM_ID
);

const tx = await program.methods
  .initializeDao(500, 86400) // 5% penalty, 24h rounds
  .accounts({
    admin,
    daoConfig,
    mint,
    vaultAuthority,
    vaultTokenAccount,
    tokenProgram: TOKEN_2022_PROGRAM_ID,
  })
  .rpc();

console.log("DAO initialized:", tx);


});
});