import { Connection, PublicKey, Keypair, SystemProgram } from '@solana/web3.js';
import { Program, Provider, web3 } from '@coral-xyz/anchor';
import { Group8 } from "../target/types/group8";
import idl from './group8.json';

// Function to test the lockNFT functionality
async function testLockNFT() {
    const anchor = require("@coral-xyz/anchor");
    // Connection to devnet
    const connection = new Connection('https://api.devnet.solana.com', 'confirmed');
    const provider = new anchor.AnchorProvider(connection, {
      preflightCommitment: 'processed',
    }, {});
  
    // Load the program
    const programId = new PublicKey('GVyKhCt25xvgXbjGyGq8WRjPbvUD1TToyyQPpRYZp8wa');
    const program = new anchor.Program(idl, programId, provider);
  
    // Required input accounts
    const sender = Keypair.generate();
    const senderTokenAccount = Keypair.generate();
    const mint = Keypair.generate();
    const nft = Keypair.generate();
    const lockingTokenAccount = Keypair.generate();
    const programAccount = Keypair.generate();
  
    // lockNFT function call
    try {
      await program.rpc.lockNFT({
        accounts: {
          sender: sender.publicKey,
          senderTokenAccount: senderTokenAccount.publicKey,
          nft: nft.publicKey,
          lockingTokenAccount: lockingTokenAccount.publicKey,
          program: programAccount.publicKey,
          tokenProgram: new PublicKey('TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA'),
          rent: web3.SYSVAR_RENT_PUBKEY,
          systemProgram: SystemProgram.programId,
          mint: mint.publicKey,
        },
        signers: [sender, senderTokenAccount, mint, nft, lockingTokenAccount, programAccount],
      });
  
      console.log('NFT locked successfully!');
    } catch (error) {
      console.error('Error locking NFT:', error);
    }
  }
  
  // Button to run the test
  const runTestButton = document.createElement('button');
  runTestButton.innerText = 'Run LockNFT Test';
  runTestButton.addEventListener('click', () => {
    testLockNFT();
  });
  
  document.body.appendChild(runTestButton);