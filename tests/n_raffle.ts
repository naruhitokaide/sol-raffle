import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { NRaffle } from "../target/types/n_raffle";
import { SystemProgram, Keypair, PublicKey, Transaction, SYSVAR_RENT_PUBKEY, SYSVAR_CLOCK_PUBKEY, LAMPORTS_PER_SOL, Connection } from "@solana/web3.js";
import { TOKEN_PROGRAM_ID, createAccount, createAssociatedTokenAccount, getAssociatedTokenAddress , ASSOCIATED_TOKEN_PROGRAM_ID,createMint, mintTo, mintToChecked, getAccount, getMint, getAssociatedTokenAddressSync,  } from "@solana/spl-token";


describe("n_raffle", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.NRaffle as Program<NRaffle>;

  const provdier = anchor.AnchorProvider.env();

  const owner = Keypair.fromSecretKey(Uint8Array.from([/**/]));
  const user = Keypair.fromSecretKey(Uint8Array.from([/**/]));
  let globalState, vault: PublicKey;
  let globalStateBump, vaultBump: Number;

  //  set the pda seeds
  const GLOBAL_STATE_SEED = "GLOBAL-STATE-SEED";
  const VAULT_SEED = "VAULT-SEED";

  it("GET PDA", async() => {
    [globalState, globalStateBump] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from(GLOBAL_STATE_SEED),
      ],
      program.programId
    );

    [vault, vaultBump] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from(VAULT_SEED)
      ],
      program.programId
    );
  })

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.rpc.initialize(
      {
        accounts: {
          owner: owner.publicKey,
          globalState,
          vault,
          systemProgram: SystemProgram.programId
        },
        signers: [owner]
      }
    );
    console.log("Your transaction signature", tx);
  });

  it("Deposit sol", async() => {
    const deposit_amount = 1 * LAMPORTS_PER_SOL; 
    try {
      const depsit_tx = await program.rpc.depositSol(
        new anchor.BN(deposit_amount),
        {
          accounts: {
            user: user.publicKey,
            globalState,
            vault,
            systemProgram: SystemProgram.programId
          },
          signers: [user]
        }
      );
      console.log("tx->", depsit_tx);
    } catch (error) {
      console.log(error);
    }
  }); 
  it("withdraw sol", async() => {
    const withdraw_amount = 1 * LAMPORTS_PER_SOL; 
    try {
      const depsit_tx = await program.rpc.withdrawPool(
        new anchor.BN(withdraw_amount),
        {
          accounts: {
            owner: owner.publicKey,
            to: user.publicKey,
            globalState,
            vault,
            systemProgram: SystemProgram.programId
          },
          signers: [owner]
        }
      );
      console.log("tx->", depsit_tx);
    } catch (error) {
      console.log(error);
    }
  }); 
});
