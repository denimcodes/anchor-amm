import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorAmm } from "../target/types/anchor_amm";
import {
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
  SystemProgram,
} from "@solana/web3.js";
import {
  Account,
  createMint,
  getAssociatedTokenAddress,
  getAssociatedTokenAddressSync,
  getOrCreateAssociatedTokenAccount,
  mintTo,
} from "@solana/spl-token";
import { randomBytes } from "crypto";
import { fail } from "assert";

describe("anchor-amm", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const provider = anchor.getProvider();
  const connection = provider.connection;

  const program = anchor.workspace.anchorAmm as Program<AnchorAmm>;

  const seed = new anchor.BN(randomBytes(8));
  const [config] = PublicKey.findProgramAddressSync(
    [Buffer.from("config"), seed.toArrayLike(Buffer, "le", 8)],
    program.programId
  );
  const user = Keypair.generate();
  const mintX = Keypair.generate();
  const mintY = Keypair.generate();
  const [mintLp] = PublicKey.findProgramAddressSync(
    [Buffer.from("lp"), config.toBuffer()],
    program.programId
  );
  const vaultX = getAssociatedTokenAddressSync(mintX.publicKey, config, true);
  const vaultY = getAssociatedTokenAddressSync(mintY.publicKey, config, true);
  const userLp = getAssociatedTokenAddressSync(mintLp, user.publicKey);
  let userX: PublicKey;
  let userY: PublicKey;

  const confirm = async (signature: string): Promise<string> => {
    const block = await connection.getLatestBlockhash();
    await connection.confirmTransaction({
      signature,
      ...block,
    });
    return signature;
  };

  const log = async (signature: string): Promise<string> => {
    console.log(
      `Your transaction signature: https://explorer.solana.com/transaction/${signature}?cluster=custom&customUrl=${connection.rpcEndpoint}`
    );
    return signature;
  };

  before(async () => {
    const signature = await connection.requestAirdrop(
      user.publicKey,
      10 * LAMPORTS_PER_SOL
    );
    await confirm(signature);

    await createMint(
      connection,
      user,
      user.publicKey,
      user.publicKey,
      6,
      mintX
    );
    await createMint(
      connection,
      user,
      user.publicKey,
      user.publicKey,
      6,
      mintY
    );

    userX = (
      await getOrCreateAssociatedTokenAccount(
        connection,
        user,
        mintX.publicKey,
        user.publicKey
      )
    ).address;
    userY = (
      await getOrCreateAssociatedTokenAccount(
        connection,
        user,
        mintY.publicKey,
        user.publicKey
      )
    ).address;

    await mintTo(
      connection,
      user,
      mintX.publicKey,
      userX,
      user.publicKey,
      1000 * Math.pow(10, 6)
    );
    await mintTo(
      connection,
      user,
      mintY.publicKey,
      userY,
      user.publicKey,
      1000 * Math.pow(10, 6)
    );
  });

  it("init config", async () => {
    const fee = 10;
    program.methods
      .initialize(seed, fee, user.publicKey)
      .accounts({
        admin: user.publicKey,
        mintX: mintX.publicKey,
        mintY: mintY.publicKey,
        // @ts-ignore
        vaultX,
        vaultY,
        mintLp,
        config,
      })
      .rpc()
      .then(confirm)
      .catch(console.error);
  });

  it("deposit tokens", async () => {
    const amount = new anchor.BN(1000);
    const maxX = new anchor.BN(500);
    const maxY = new anchor.BN(500);

    program.methods
      .deposit(amount, maxX, maxY)
      .accounts({
        user: user.publicKey,
        // @ts-ignore
        mintX: mintX.publicKey,
        mintY: mintY.publicKey,
        mintLp,
        config,
        vaultX,
        vaultY,
        userX,
        userY,
        userLp,
      })
      .rpc()
      .then(confirm)
      .catch(console.error);
  });

  it("swap x", async () => {
    const isX = true;
    const amount = new anchor.BN(500);
    const min = new anchor.BN(500);
    program.methods
      .swap(isX, amount, min)
      .accounts({
        user: user.publicKey,
        mintX: mintX.publicKey,
        mintY: mintY.publicKey,
        // @ts-ignore
        mintLp,
        config,
        vaultX,
        vaultY,
        userX,
        userY,
        userLp,
      })
      .rpc()
      .then(confirm)
      .then(log)
      .catch(console.error);
  });
  it("swap y", async () => {
    const isX = false;
    const amount = new anchor.BN(500);
    const min = new anchor.BN(500);
    program.methods
      .swap(isX, amount, min)
      .accounts({
        user: user.publicKey,
        mintX: mintX.publicKey,
        mintY: mintY.publicKey,
        // @ts-ignore
        mintLp,
        config,
        vaultX,
        vaultY,
        userX,
        userY,
        userLp,
      })
      .rpc()
      .then(confirm)
      .then(log)
      .catch(console.error);
  });
  it("withdraw", async () => {
    const amount = new anchor.BN(1000);
    const minX = new anchor.BN(500);
    const minY = new anchor.BN(500);

    program.methods
      .withdraw(amount, minX, minY)
      .accounts({
        user: user.publicKey,
        // @ts-ignore
        mintX: mintX.publicKey,
        mintY: mintY.publicKey,
        mintLp,
        config,
        vaultX,
        vaultY,
        userX,
        userY,
        userLp,
      })
      .rpc()
      .then(confirm)
      .catch(console.error);
  });
});
