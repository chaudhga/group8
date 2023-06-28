import React, { useState } from 'react';
import './App.css';
import { Connection, PublicKey, Keypair, SystemProgram } from '@solana/web3.js';
import { Program, Provider, web3 } from '@coral-xyz/anchor';
import idl from './group8.json';

function App() {
  const [programIdInput, setProgramIdInput] = useState('');
  const [networkInput, setNetworkInput] = useState('');
  const [nftInput, setNftInput] = useState('');
  const [result, setResult] = useState('');

  const testLockNFT = async (programId: string, network: string, nft: PublicKey) => {
    const anchor = require("@coral-xyz/anchor");

    // Connection to the specified network
    const connection = new Connection(network, 'confirmed');
    const provider = new anchor.AnchorProvider(connection, {
      preflightCommitment: 'processed',
    }, {});

    // Load the program
    const programIdPublicKey = new PublicKey(programId);
    const program = new anchor.Program(idl, programIdPublicKey, provider);

    // Required input accounts
    const sender = Keypair.generate();
    const senderTokenAccount = Keypair.generate();
    const mint = Keypair.generate();
    const lockingTokenAccount = Keypair.generate();
    const programAccount = Keypair.generate();

    // lockNFT function call
    try {
      await program.rpc.lockNFT({
        accounts: {
          sender: sender.publicKey,
          senderTokenAccount: senderTokenAccount.publicKey,
          nft: nft,
          lockingTokenAccount: lockingTokenAccount.publicKey,
          program: programAccount.publicKey,
          tokenProgram: new PublicKey('TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA'),
          rent: web3.SYSVAR_RENT_PUBKEY,
          systemProgram: SystemProgram.programId,
          mint: mint.publicKey,
        },
        signers: [sender, senderTokenAccount, mint, nft, lockingTokenAccount, programAccount],
      });

      setResult('NFT locked successfully!');
    } catch (error) {
      console.error('Error locking NFT:', error);
      setResult('Failed to lock NFT.');
    }
  };

  const onSubmit = () => {
    testLockNFT(programIdInput, networkInput, new PublicKey(nftInput));
  };

  return (
    <div className="App">
      <header className="App-header">
        <div>
          <label htmlFor="programIdInput">Program ID:</label>
          <input
            id="programIdInput"
            value={programIdInput}
            onChange={(e) => setProgramIdInput(e.target.value)}
          />
        </div>
        <div>
          <label htmlFor="networkInput">Network:</label>
          <input
            id="networkInput"
            value={networkInput}
            onChange={(e) => setNetworkInput(e.target.value)}
          />
        </div>
        <div>
          <label htmlFor="nftInput">NFT Public Key:</label>
          <input
            id="nftInput"
            value={nftInput}
            onChange={(e) => setNftInput(e.target.value)}
          />
        </div>
        <button onClick={onSubmit}>Run LockNFT Test</button>
        <div>{result}</div>
      </header>
    </div>
  );
}

export default App;