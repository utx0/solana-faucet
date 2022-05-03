import * as anchor from "@project-serum/anchor";
import { Program, web3 } from "@project-serum/anchor";
import { HydraswapSolanaFaucet } from "../target/types/hydraswap_solana_faucet";
import {
  AccountLayout,
  createMint,
  getOrCreateAssociatedTokenAccount,
  createInitializeAccountInstruction,
  mintTo,
  TOKEN_PROGRAM_ID,
  getAssociatedTokenAddress,
  RawAccount,
  ASSOCIATED_TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import { findProgramAddressSync } from "@project-serum/anchor/dist/cjs/utils/pubkey";
import { assert } from "chai";

describe("hydraswap-solana-faucet", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace
    .HydraswapSolanaFaucet as Program<HydraswapSolanaFaucet>;
  const connection = anchor.getProvider().connection;

  // user interacting with the smart contract
  const user = web3.Keypair.generate();
  let [userPda, userBump] = findProgramAddressSync(
    [anchor.utils.bytes.utf8.encode("user_meta_v0"), user.publicKey.toBuffer()],
    program.programId
  );

  // perform prerequisites for the tests
  before(async () => {
    // request airdrop of 10 SOL for user
    // being greedy as running tests on local solana-validator
    await connection.confirmTransaction(
      await connection.requestAirdrop(
        user.publicKey,
        10 * web3.LAMPORTS_PER_SOL
      )
    );
  });

  // User should be allowed to mint a token
  // if it's his/her first time
  it("(+) Allow first mint", async () => {
    try {
      const faucet_mint_seed = "hydraswap-faucet-mint";

      let [mintPda, otherBump] = findProgramAddressSync(
        [
          anchor.utils.bytes.utf8.encode(faucet_mint_seed),
          user.publicKey.toBuffer(),
          anchor.utils.bytes.utf8.encode("mint-one"),
        ],
        program.programId
      );

      let mintPdaTokenAddress = await getAssociatedTokenAddress(
        mintPda,
        mintPda,
        true
      );

      await program.methods
        .createMintAndVault("mint-one", new anchor.BN(2000000))
        .accounts({
          systemProgram: anchor.web3.SystemProgram.programId,
          user: user.publicKey,
          userMeta: userPda,
          mint: mintPda,
          rent: anchor.web3.SYSVAR_RENT_PUBKEY,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          vault: mintPdaTokenAddress,
        })
        .signers([user])
        .rpc();
    } catch (error) {
      console.log(error);
    }

    let userMetaAccount = await program.account.userMeta.fetch(userPda);

    let details = userMetaAccount.mintDetails as Array<any>;
    assert.equal(1, details.length);
  });

  // User should also be allowed to mint a token
  // if it's his/her second time
  it("(+) Allow second mint", async () => {
    try {
      const faucet_mint_seed = "hydraswap-faucet-mint";

      let [mintPda, otherBump] = findProgramAddressSync(
        [
          anchor.utils.bytes.utf8.encode(faucet_mint_seed),
          user.publicKey.toBuffer(),
          anchor.utils.bytes.utf8.encode("mint-two"),
        ],
        program.programId
      );

      let mintPdaTokenAddress = await getAssociatedTokenAddress(
        mintPda,
        mintPda,
        true
      );

      await program.methods
        .createMintAndVault("mint-two", new anchor.BN(2000000))
        .accounts({
          systemProgram: anchor.web3.SystemProgram.programId,
          user: user.publicKey,
          userMeta: userPda,
          mint: mintPda,
          rent: anchor.web3.SYSVAR_RENT_PUBKEY,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          vault: mintPdaTokenAddress,
        })
        .signers([user])
        .rpc();
    } catch (error) {
      console.log(error);
    }

    let userMetaAccount = await program.account.userMeta.fetch(userPda);

    let details = userMetaAccount.mintDetails as Array<any>;
    assert.equal(2, details.length);
  });

  // Minting more than twice
  // must not be allowed
  it("(-) Disallow third mint", async () => {
    try {
      const faucet_mint_seed = "hydraswap-faucet-mint";

      let [mintPda, otherBump] = findProgramAddressSync(
        [
          anchor.utils.bytes.utf8.encode(faucet_mint_seed),
          user.publicKey.toBuffer(),
          anchor.utils.bytes.utf8.encode("mint-three"),
        ],
        program.programId
      );

      let mintPdaTokenAddress = await getAssociatedTokenAddress(
        mintPda,
        mintPda,
        true
      );

      await program.methods
        .createMintAndVault("mint-three", new anchor.BN(2000000))
        .accounts({
          systemProgram: anchor.web3.SystemProgram.programId,
          user: user.publicKey,
          userMeta: userPda,
          mint: mintPda,
          rent: anchor.web3.SYSVAR_RENT_PUBKEY,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          vault: mintPdaTokenAddress,
        })
        .signers([user])
        .rpc();
    } catch (error) {
      assert.equal(
        "Your minting quota exhausted (2/2)",
        error.error.errorMessage
      );
      return;
    }

    assert.fail("This test was supposed to fail");
  });

  // Should allow requesting token
  // once a day (24 hours) only
  it("(+) Should get tokens for first time in day", async () => {
    try {
      const faucet_mint_seed = "hydraswap-faucet-mint";

      let [mintPda, otherBump] = findProgramAddressSync(
        [
          anchor.utils.bytes.utf8.encode(faucet_mint_seed),
          user.publicKey.toBuffer(),
          anchor.utils.bytes.utf8.encode("mint-one"),
        ],
        program.programId
      );

      let mintPdaTokenAddress = await getAssociatedTokenAddress(
        mintPda,
        mintPda,
        true
      );

      let userTokenAddress = await getAssociatedTokenAddress(
        mintPda,
        user.publicKey
      );

      await program.methods
        .requestTokens()
        .accounts({
          user: user.publicKey,
          userMeta: userPda,
          mint: mintPda,
          vault: mintPdaTokenAddress,
          userTokenAccount: userTokenAddress,
          systemProgram: anchor.web3.SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        })
        .signers([user])
        .rpc();

      let [_, tokenBalance] = await fetchAndDecodeTokenAccount(
        userTokenAddress,
        anchor.getProvider()
      );

      assert.equal(tokenBalance, "10000");
    } catch (error) {
      console.log(error);
    }

    let userMetaAccount = await program.account.userMeta.fetch(userPda);

    let details = userMetaAccount.mintDetails as Array<any>;
    assert.equal(2, details.length);
    assert.notEqual(0, details[0].last_requested);
  });

  // User must not be allowed
  // to request tokens
  it("(-) Disallow request token for second time in day", async () => {
    try {
      const faucet_mint_seed = "hydraswap-faucet-mint";

      let [mintPda, otherBump] = findProgramAddressSync(
        [
          anchor.utils.bytes.utf8.encode(faucet_mint_seed),
          user.publicKey.toBuffer(),
          anchor.utils.bytes.utf8.encode("mint-one"),
        ],
        program.programId
      );

      let mintPdaTokenAddress = await getAssociatedTokenAddress(
        mintPda,
        mintPda,
        true
      );

      let userTokenAddress = await getAssociatedTokenAddress(
        mintPda,
        user.publicKey
      );

      await program.methods
        .requestTokens()
        .accounts({
          user: user.publicKey,
          userMeta: userPda,
          mint: mintPda,
          vault: mintPdaTokenAddress,
          userTokenAccount: userTokenAddress,
          systemProgram: anchor.web3.SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        })
        .signers([user])
        .rpc();
    } catch (error) {
      assert.equal(
        "Token request Quota for the day has been exhausted (1/1)",
        error.error.errorMessage
      );
      return;
    }

    assert.fail("This test was supposed to fail");
  });
});

/**
 * A utility function to
 * fetch given account and decode it as Token account
 */
const fetchAndDecodeTokenAccount = async (
  tokenAccountPubKey: web3.PublicKey,
  provider: anchor.Provider
): Promise<[RawAccount, string]> => {
  const tokenInfoLol = await provider.connection.getAccountInfo(
    tokenAccountPubKey
  );
  const data = Buffer.from(tokenInfoLol.data);
  const rawAccount: RawAccount = AccountLayout.decode(data);

  const amount = rawAccount.amount;
  return [rawAccount, amount.toString()];
};
