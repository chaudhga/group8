import React, { useState } from 'react';
import './App.css';
import { Connection, PublicKey, Keypair, SystemProgram } from '@solana/web3.js';
import { web3 } from '@coral-xyz/anchor';
import idl from './group8.json';

function App() {
  const [programIdInput, setProgramIdInput] = useState('GVyKhCt25xvgXbjGyGq8WRjPbvUD1TToyyQPpRYZp8wa');
  const [networkInput, setNetworkInput] = useState('https://api.devnet.solana.com');
  const [nftInput, setNftInput] = useState('');
  const [result, setResult] = useState('');

  const testLockNFT = async (programId: string, network: string, nft: PublicKey) => {
    // ... (rest of the code)
  };

  const onSubmit = () => {
    testLockNFT(programIdInput, networkInput, new PublicKey(nftInput));
  };

  return (
    <div className="App">
      <header className="App-header">
        <h1>LockNFT Test</h1>
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
          <select id="networkInput" value={networkInput} onChange={(e) => setNetworkInput(e.target.value)}>
            <option value="https://api.devnet.solana.com">Devnet</option>
            <option value="https://api.testnet.solana.com">Testnet</option>
            <option value="https://api.mainnet-beta.solana.com">Mainnet Beta</option>
          </select>
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
        <div className="instructions">
          <h2>Instructions to create NFT Public Key for testing:</h2>
          <ol>
            <li>Create an NFT using a tool like Solana's Metaplex.</li>
            <li>After creating the NFT, you will receive a public key representing the NFT.</li>
            <li>Copy the public key and paste it into the "NFT Public Key" field above.</li>
          </ol>
        </div>
      </header>
    </div>
  );
}

export default App;