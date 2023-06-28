import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Group8 } from "../target/types/group8";
import { clusterApiUrl, Connection, Keypair, LAMPORTS_PER_SOL, Transaction } from '@solana/web3.js';
import { TOKEN_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID, mintTo } from "@solana/spl-token";
import * as splToken from '@solana/spl-token'; 
import { NATIVE_MINT, createAssociatedTokenAccountInstruction, createMint, getAssociatedTokenAddress,
  getOrCreateAssociatedTokenAccount, createSetAuthorityInstruction, AuthorityType } from '@solana/spl-token';


describe("group8", () => {
  const web3 = require('@solana/web3.js');
  const Token = require("@solana/spl-token").Token;

  
  let payer = Keypair.generate();
  let ownerAccount = Keypair.generate();
  let mintAuthority = Keypair.generate();
  let connection = new web3.Connection("http://localhost:8899");
  
  let token;
  let mintAuthorityBalance;
  let payerBalance;

  // Set up the environment before running the tests
  before(async () => {

    // Configure the client to use the local cluster.
    anchor.setProvider(anchor.AnchorProvider.env());

    mintAuthorityBalance = await connection.getBalance(mintAuthority.publicKey);
    payerBalance = await connection.getBalance(payer.publicKey);

    // Fund the mintAuthority account
    if (mintAuthorityBalance <= 10){
      await connection.requestAirdrop(mintAuthority.publicKey, 200 * LAMPORTS_PER_SOL);
      await new Promise((r) => setTimeout(r, 15000));
    }


    console.log("Mint Authority SOL balance:", await connection.getBalance(mintAuthority.publicKey));

    if (payerBalance <= 10){
      // Fund the payer account
      await connection.requestAirdrop(payer.publicKey, 200 * LAMPORTS_PER_SOL);
      await new Promise((r) => setTimeout(r, 15000));
    }

    console.log("Payer SOL balance:", await connection.getBalance(payer.publicKey));

    const mint = await createMint(
      connection,
      mintAuthority,
      mintAuthority.publicKey,
      null,
      0
    );

    console.log(
      "new SOL balance of minter after token creation activity",
      await connection.getBalance(mintAuthority.publicKey)
    );
    console.log(mint.toBase58());

  });


  it("Mints an NFT and assigns it to the owner account", async () => {
      // Create the token type with zero decimal places
      const mint = await createMint(
        connection,
        payer,
        mintAuthority.publicKey,
        null,
        0
      );

      // Create an account to hold tokens of this new type
      const associatedTokenAccount = await getOrCreateAssociatedTokenAccount(
        connection,
        payer,
        mint,
        payer.publicKey
      );

      await mintTo(
        connection,
        payer,
        mint,
        associatedTokenAccount.address,
        payer,
        1
      );

      
      let transaction = new Transaction()
      .add(createSetAuthorityInstruction(
        mint,
        payer.publicKey,
        AuthorityType.MintTokens,
        null
      ));

      await web3.sendAndConfirmTransaction(connection, transaction, [payer]);

  });

  it("Is initialized!", async () => {
    // Add your test here.
    const program = anchor.workspace.Group8 as Program<Group8>;
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });

  // TODO: Add your fractionalize contract testing code here
});
